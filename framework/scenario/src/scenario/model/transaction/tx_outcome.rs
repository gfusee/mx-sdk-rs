use std::fmt::{Display, Formatter};
use base64::Engine;
use multiversx_sdk::data::transaction::{ApiLogs, ApiSmartContractResult, Events};

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";

#[derive(PartialEq, Debug, Clone)]
pub enum TransactionsOutcomeParserError {
    ErrorInTransaction { data: String, message: String },
    NoEventOfType { r#type: String },
    MoreThanOneEventOfType { r#type: String },
    CannotExtractTokenIdentifierFromResults { expected_issuance_start_data: String },
    NoResultToExtractTokenIdentifier { expected_issuance_start_data: String },
    CannotDecodeBase64StringToUTF8 { base64: String },
    CannotDecodeHexStringToUTF8 { hex: String },
}

impl Display for TransactionsOutcomeParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionsOutcomeParserError::ErrorInTransaction { data, message } => {
                write!(f, "Encountered signalError: {data} {message}")
            },
            TransactionsOutcomeParserError::NoEventOfType { r#type } => {
                write!(f, "Cannot find event of type {type}")
            },
            TransactionsOutcomeParserError::MoreThanOneEventOfType { r#type } => {
                write!(f, "Found more than one event of type {type}")
            },
            TransactionsOutcomeParserError::CannotExtractTokenIdentifierFromResults { expected_issuance_start_data } => {
                write!(f, "Cannot extract token identifier from results with expected issuance start data: {expected_issuance_start_data}")
            },
            TransactionsOutcomeParserError::NoResultToExtractTokenIdentifier { expected_issuance_start_data } => {
                write!(f, "No result to extract issued token identifier with expected issuance start data: {expected_issuance_start_data}")
            },
            TransactionsOutcomeParserError::CannotDecodeBase64StringToUTF8 { base64 } => {
                write!(f, "Cannot decode base64 string to utf8: {base64}")
            },
            TransactionsOutcomeParserError::CannotDecodeHexStringToUTF8 { hex } => {
                write!(f, "Cannot decode hex string to utf8: {hex}")
            },
        }
    }
}

pub trait TransactionOutcome {
    fn get_smart_contract_results(self) -> Vec<ApiSmartContractResult>;
    fn get_transaction_logs(self) -> Option<ApiLogs>;

    fn get_transaction_logs_ref(&self) -> &Option<ApiLogs>;
    fn get_smart_contract_results_ref(&self) -> &[ApiSmartContractResult];

    fn ensure_no_error(&self) -> Result<(), TransactionsOutcomeParserError> {
        let Some(logs) = self.get_transaction_logs_ref().as_ref() else {
            return Ok(())
        };

        for event in logs.events.iter() {
            if event.identifier == "signalError" {
                let data_base64 = event.data.clone().unwrap_or_default();
                let message_base64 = if let Some(topics) = event.topics.as_ref() {
                    topics.get(1).cloned().unwrap_or_default()
                } else {
                    "".to_string()
                };

                let message = base64_to_utf8(&message_base64)?;
                let data = base64_to_utf8(&data_base64)?;

                return Err(TransactionsOutcomeParserError::ErrorInTransaction { message, data })
            }
        }

        Ok(())
    }

    fn parse_issue_fungible(&self) -> Result<String, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        extract_token_identifier_from_scrs(
            self.get_smart_contract_results_ref(),
            "issue@"
        )
    }

    fn parse_issue_non_fungible(&self) -> Result<String, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        extract_token_identifier_from_scrs(
            self.get_smart_contract_results_ref(),
            "issueNonFungible@"
        )
    }

    fn parse_issue_semi_fungible(&self) -> Result<String, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        extract_token_identifier_from_scrs(
            self.get_smart_contract_results_ref(),
            "issueSemiFungible@"
        )
    }

    fn parse_register_meta_esdt(&self) -> Result<String, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        extract_token_identifier_from_scrs(
            self.get_smart_contract_results_ref(),
            "registerMetaESDT@"
        )
    }

}

fn find_single_event_by_identifier(api_logs: &Option<ApiLogs>, identifier: &str) -> Result<Events, TransactionsOutcomeParserError> {
    let Some(logs) = api_logs else {
        return Err(TransactionsOutcomeParserError::NoEventOfType { r#type: identifier.to_string() })
    };

    let mut filtered: Vec<&Events> = logs.events.iter()
        .filter(|e| e.identifier == identifier)
        .collect();

    if filtered.is_empty() {
        return Err(TransactionsOutcomeParserError::NoEventOfType { r#type: identifier.to_string() })
    }

    if filtered.len() > 1 {
        return Err(TransactionsOutcomeParserError::MoreThanOneEventOfType { r#type: identifier.to_string() })
    }

    Ok(filtered.remove(0).clone())
}

fn extract_token_identifier_from_scrs(scrs: &[ApiSmartContractResult], expected_issuance_start_data: &str) -> Result<String, TransactionsOutcomeParserError> {
    for scr in scrs {
        if scr.sender.to_string() != SYSTEM_SC_BECH32 {
            continue;
        }

        let Some(prev_tx) = scrs.iter().find(|e| e.hash == scr.prev_tx_hash) else {
            continue;
        };

        if !prev_tx.data.starts_with(expected_issuance_start_data) {
            continue;
        }

        if scr.data.starts_with("ESDTTransfer@") {
            let opt_encoded_tid = scr.data.split('@').nth(1);
            let Some(encoded_tid) = opt_encoded_tid else {
                return Err(TransactionsOutcomeParserError::CannotExtractTokenIdentifierFromResults { expected_issuance_start_data: expected_issuance_start_data.to_string() });
            };

            return hex_to_utf8(encoded_tid)
        } else if scr.data.starts_with("@00@") {
            let opt_encoded_tid = scr.data.split('@').nth(2);
            let Some(encoded_tid) = opt_encoded_tid else {
                return Err(TransactionsOutcomeParserError::CannotExtractTokenIdentifierFromResults { expected_issuance_start_data: expected_issuance_start_data.to_string() });
            };

            return hex_to_utf8(encoded_tid)
        }
    }

    Err(TransactionsOutcomeParserError::NoResultToExtractTokenIdentifier { expected_issuance_start_data: expected_issuance_start_data.to_string() })
}

fn base64_to_utf8(base64: &str) -> Result<String, TransactionsOutcomeParserError> {
    let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(base64) else {
        return Err(TransactionsOutcomeParserError::CannotDecodeBase64StringToUTF8 { base64: base64.to_string() })
    };

    let Ok(result) = String::from_utf8(decoded) else {
        return Err(TransactionsOutcomeParserError::CannotDecodeBase64StringToUTF8 { base64: base64.to_string() })
    };

    Ok(result)
}

fn hex_to_utf8(hex: &str) -> Result<String, TransactionsOutcomeParserError> {
    let Ok(decoded) = hex::decode(hex) else {
        return Err(TransactionsOutcomeParserError::CannotDecodeHexStringToUTF8 { hex: hex.to_string() })
    };

    let Ok(result) = String::from_utf8(decoded) else {
        return Err(TransactionsOutcomeParserError::CannotDecodeHexStringToUTF8 { hex: hex.to_string() })
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use multiversx_sdk::data::address::Address;
    use multiversx_sdk::data::transaction::{ApiLogs, Events, TransactionInfo, TransactionOnNetwork};
    use crate::scenario_model::transaction::tx_outcome::{find_single_event_by_identifier, TransactionOutcome, TransactionsOutcomeParserError};
    use crate::scenario_model::TxResponse;

    #[test]
    fn test_ensure_no_error_with_successful_tx() {
        let ensure_no_error_result = get_swap_tx().ensure_no_error();

        assert_eq!(Ok(()), ensure_no_error_result)
    }

    #[test]
    fn test_ensure_no_error_with_failed_tx() {
        let ensure_no_error_result = get_error_intra_shard_tx().ensure_no_error();

        let expected_error = TransactionsOutcomeParserError::ErrorInTransaction {
            data: "@75736572206572726f72".to_string(),
            message: "wrong number of arguments".to_string(),
        };

        assert_eq!(Err(expected_error), ensure_no_error_result)
    }

    #[test]
    fn test_find_single_event_by_identifier_event_exists() {
        let events = vec![
            Events {
                address: Address::from_bytes(Default::default()),
                identifier: "test".to_string(),
                topics: None,
                data: None,
            },
            Events {
                address: Address::from_bytes(Default::default()),
                identifier: "issue".to_string(),
                topics: None,
                data: None,
            },
            Events {
                address: Address::from_bytes(Default::default()),
                identifier: "test".to_string(),
                topics: None,
                data: None,
            },
        ];

        let api_logs = ApiLogs {
            address: Address::from_bytes(Default::default()),
            events,
        };

        let result = find_single_event_by_identifier(&Some(api_logs), "issue");

        let expected_event = Events {
            address: Address::from_bytes(Default::default()),
            identifier: "issue".to_string(),
            topics: None,
            data: None,
        };

        assert_eq!(Ok(expected_event), result);
    }

    #[test]
    fn test_find_single_event_by_identifier_no_event_at_all() {
        let api_logs = None;

        let result = find_single_event_by_identifier(&api_logs, "issue");

        let expected_error = TransactionsOutcomeParserError::NoEventOfType { r#type: "issue".to_string() };

        assert_eq!(Err(expected_error), result);
    }

    #[test]
    fn test_find_single_event_by_identifier_no_event_found() {
        let events = vec![
            Events {
                address: Address::from_bytes(Default::default()),
                identifier: "test".to_string(),
                topics: None,
                data: None,
            },
            Events {
                address: Address::from_bytes(Default::default()),
                identifier: "issue".to_string(),
                topics: None,
                data: None,
            },
            Events {
                address: Address::from_bytes(Default::default()),
                identifier: "test".to_string(),
                topics: None,
                data: None,
            },
        ];

        let api_logs = ApiLogs {
            address: Address::from_bytes(Default::default()),
            events,
        };

        let result = find_single_event_by_identifier(&Some(api_logs), "unknown");

        let expected_error = TransactionsOutcomeParserError::NoEventOfType { r#type: "unknown".to_string() };

        assert_eq!(Err(expected_error), result);
    }

    #[test]
    fn test_find_single_event_by_identifier_more_than_one_event_found() {
        let events = vec![
            Events {
                address: Address::from_bytes(Default::default()),
                identifier: "test".to_string(),
                topics: None,
                data: None,
            },
            Events {
                address: Address::from_bytes(Default::default()),
                identifier: "issue".to_string(),
                topics: None,
                data: None,
            },
            Events {
                address: Address::from_bytes(Default::default()),
                identifier: "test".to_string(),
                topics: None,
                data: None,
            },
        ];

        let api_logs = ApiLogs {
            address: Address::from_bytes(Default::default()),
            events,
        };

        let result = find_single_event_by_identifier(&Some(api_logs), "test");

        let expected_error = TransactionsOutcomeParserError::MoreThanOneEventOfType { r#type: "test".to_string() };

        assert_eq!(Err(expected_error), result);
    }

    #[test]
    fn test_parse_issue_fungible_valid() {
        let parsed_token = get_tx_indirect_issue_fungible_tx().parse_issue_fungible();

        let expected: Result<String, TransactionsOutcomeParserError> = Ok("EGLDMEX-95c6d5".to_string());

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_issue_fungible_invalid() {
        let parsed_token = get_swap_tx().parse_issue_fungible();

        let expected: Result<String, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoResultToExtractTokenIdentifier { expected_issuance_start_data: "issue@".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_issue_non_fungible_valid() {
        let parsed_token = get_indirect_issue_non_fungible_tx().parse_issue_non_fungible();

        let expected: Result<String, TransactionsOutcomeParserError> = Ok("GEN-868593".to_string());

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_issue_non_fungible_invalid() {
        let parsed_token = get_swap_tx().parse_issue_non_fungible();

        let expected: Result<String, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoResultToExtractTokenIdentifier { expected_issuance_start_data: "issueNonFungible@".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_issue_semi_fungible_valid() {
        let parsed_token = get_indirect_issue_semi_fungible_tx().parse_issue_semi_fungible();

        let expected: Result<String, TransactionsOutcomeParserError> = Ok("DOPETEST-77200c".to_string());

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_issue_semi_fungible_invalid() {
        let parsed_token = get_swap_tx().parse_issue_semi_fungible();

        let expected: Result<String, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoResultToExtractTokenIdentifier { expected_issuance_start_data: "issueSemiFungible@".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_register_meta_esdt_valid() {
        let parsed_token = get_indirect_register_meta_esdt_tx().parse_register_meta_esdt();

        let expected: Result<String, TransactionsOutcomeParserError> = Ok("AVASH-7d8b5d".to_string());

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_register_meta_esdt_invalid() {
        let parsed_token = get_swap_tx().parse_register_meta_esdt();

        let expected: Result<String, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoResultToExtractTokenIdentifier { expected_issuance_start_data: "registerMetaESDT@".to_string() });

        assert_eq!(parsed_token, expected);
    }

    fn get_swap_tx() -> TxResponse {
        let data = r#"
        {
          "data": {
            "transaction": {
              "type": "normal",
              "processingTypeOnSource": "SCInvoking",
              "processingTypeOnDestination": "SCInvoking",
              "hash": "b5fafeaa4c5062aa5a9ac999b762b4b1335e903d972fc18f35fdeff925b53424",
              "nonce": 3649,
              "round": 2637554,
              "epoch": 1073,
              "value": "200000000000000000",
              "receiver": "erd1qqqqqqqqqqqqqpgqqkwzsxkjc83vlfex9dmznwm7tjvxlqqkpauqx0n782",
              "sender": "erd1wh9c0sjr2xn8hzf02lwwcr4jk2s84tat9ud2kaq6zr7xzpvl9l5q8awmex",
              "gasPrice": 1000000000,
              "gasLimit": 4200000,
              "gasUsed": 2156471,
              "data": "d3JhcEVnbGQ=",
              "signature": "9462686fb48817f834a6ef3596966272feb248ba9bb9e274ee75d5fd9f11925d0fa294d289cecfaec76da3631fea0695910d115789994df30e9ff9e997a31f00",
              "sourceShard": 0,
              "destinationShard": 0,
              "blockNonce": 2572683,
              "blockHash": "adda14bb275177aca88ce0e0d066bcfbe53cac99f0cc488e09a97f65afc8e96b",
              "notarizedAtSourceInMetaNonce": 2575640,
              "NotarizedAtSourceInMetaHash": "9624040cbf1236fa4ad21dc9b4dc8dce266f4660c6a4a368aaeebdadaafa3c9d",
              "notarizedAtDestinationInMetaNonce": 2575640,
              "notarizedAtDestinationInMetaHash": "9624040cbf1236fa4ad21dc9b4dc8dce266f4660c6a4a368aaeebdadaafa3c9d",
              "miniblockType": "TxBlock",
              "miniblockHash": "c96c36cad0100e05ff56e77bced312ac792a85fb48b3ce6fe4ca6109ae3175f2",
              "hyperblockNonce": 2575640,
              "hyperblockHash": "9624040cbf1236fa4ad21dc9b4dc8dce266f4660c6a4a368aaeebdadaafa3c9d",
              "timestamp": 1709825324,
              "smartContractResults": [
                {
                  "hash": "9d5a7273e1284b4db02a7433b46d6b8e821fbdd69536c7ad0cf19394cd892f29",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1wh9c0sjr2xn8hzf02lwwcr4jk2s84tat9ud2kaq6zr7xzpvl9l5q8awmex",
                  "sender": "erd1qqqqqqqqqqqqqpgqqkwzsxkjc83vlfex9dmznwm7tjvxlqqkpauqx0n782",
                  "data": "ESDTTransfer@5745474c442d613238633539@02c68af0bb140000",
                  "prevTxHash": "b5fafeaa4c5062aa5a9ac999b762b4b1335e903d972fc18f35fdeff925b53424",
                  "originalTxHash": "b5fafeaa4c5062aa5a9ac999b762b4b1335e903d972fc18f35fdeff925b53424",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "originalSender": "erd1wh9c0sjr2xn8hzf02lwwcr4jk2s84tat9ud2kaq6zr7xzpvl9l5q8awmex",
                  "tokens": [
                    "WEGLD-a28c59"
                  ],
                  "esdtValues": [
                    "200000000000000000"
                  ],
                  "operation": "ESDTTransfer"
                },
                {
                  "hash": "c6258b24c793a4cb1f69ed0af4f3d40334ee643b71984a0aec51a24de783ac30",
                  "nonce": 3650,
                  "value": 20435290000000,
                  "receiver": "erd1wh9c0sjr2xn8hzf02lwwcr4jk2s84tat9ud2kaq6zr7xzpvl9l5q8awmex",
                  "sender": "erd1qqqqqqqqqqqqqpgqqkwzsxkjc83vlfex9dmznwm7tjvxlqqkpauqx0n782",
                  "data": "@6f6b",
                  "prevTxHash": "b5fafeaa4c5062aa5a9ac999b762b4b1335e903d972fc18f35fdeff925b53424",
                  "originalTxHash": "b5fafeaa4c5062aa5a9ac999b762b4b1335e903d972fc18f35fdeff925b53424",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "operation": "transfer",
                  "isRefund": true
                }
              ],
              "logs": {
                "address": "erd1qqqqqqqqqqqqqpgqqkwzsxkjc83vlfex9dmznwm7tjvxlqqkpauqx0n782",
                "events": [
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqqkwzsxkjc83vlfex9dmznwm7tjvxlqqkpauqx0n782",
                    "identifier": "ESDTLocalMint",
                    "topics": [
                      "V0VHTEQtYTI4YzU5",
                      "",
                      "AsaK8LsUAAA="
                    ],
                    "data": null,
                    "additionalData": null
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqqkwzsxkjc83vlfex9dmznwm7tjvxlqqkpauqx0n782",
                    "identifier": "ESDTTransfer",
                    "topics": [
                      "V0VHTEQtYTI4YzU5",
                      "",
                      "AsaK8LsUAAA=",
                      "dcuHwkNRpnuJL1fc7A6ysqB6r6svGqt0GhD8YQWfL+g="
                    ],
                    "data": "RGlyZWN0Q2FsbA==",
                    "additionalData": [
                      "RGlyZWN0Q2FsbA==",
                      "RVNEVFRyYW5zZmVy",
                      "V0VHTEQtYTI4YzU5",
                      "AsaK8LsUAAA="
                    ]
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqqkwzsxkjc83vlfex9dmznwm7tjvxlqqkpauqx0n782",
                    "identifier": "completedTxEvent",
                    "topics": [
                      "tfr+qkxQYqpamsmZt2K0sTNekD2XL8GPNf3v+SW1NCQ="
                    ],
                    "data": null,
                    "additionalData": null
                  }
                ]
              },
              "status": "success",
              "operation": "transfer",
              "function": "wrapEgld",
              "initiallyPaidFee": "103380000000000",
              "fee": "82944710000000",
              "chainID": "D",
              "version": 1,
              "options": 0
            }
          },
          "error": "",
          "code": "successful"
        }
      "#;

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
            .unwrap()
            .data
            .unwrap()
            .transaction;
        TxResponse::from_network_tx(tx_on_network)
    }

    fn get_error_intra_shard_tx() -> TxResponse {
        let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "SCInvoking",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "7c870614d815bac6c4ceb43a3b6efed4e155862926a334cd05762d8604ac5abb",
                  "nonce": 556,
                  "round": 2513915,
                  "epoch": 1022,
                  "value": "50000000000000000",
                  "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                  "sender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
                  "gasPrice": 1000000000,
                  "gasLimit": 600000000,
                  "gasUsed": 600000000,
                  "data": "ZGVwbG95SGF0b21TaW1wbGVMZW5kaW5nU3RyYXRlZ3lANTU1MzQ0NDMyZDMzMzUzMDYzMzQ2NUA0MTU0NTMyZDM0NjMzMDM5MzIzMEAyMzg2ZjI2ZmMxMDAwMEAyMzg2ZjI2ZmMxMDAwMEBAM2I5YWNhMDBANDE3NTc0NmY3MzYzNjE2YzY1NTY2MTc1NmM3NDU1NTM0NDQzQDQxNTY1NTUzNDQ0M0AwMDAwMDAwMDAwMDAwMDAwMDUwMGEzYjY2NjkwMmQ1ZjRiYmYwZmY5Mzc0OGZiOTE1YWViNDMxMjRjYmY2NTA5QDAwMDAwMDAwMDAwMDAwMDAwNTAwMDFkNmViZDNhY2E2ZWVmZDg0ZDFjODUzNjlmZGU4MzFlYTIxNzIyNTY1MDlAMDAwMDAwMDAwMDAwMDAwMDA1MDBmODgzNjY4NmY3YzZjMGYxM2Q3Mjc1MTUxYjc4ZjU5ZjhmMmY3MjlhNjUwOUAwMzhkN2VhNGM2ODAwMEAyMzg2ZjI2ZmMxMDAwMEBAQDAwMDAwMDAwMDAwMDAwMDAwNTAwMGIxZTViMjQ0MzI1MDk1ODQ5ZjRlMzcxMzQ2NjFkNWJmZGNkOTI1ZTdjZWI=",
                  "signature": "afd980004e767b2bc8832e34d2882a6c528393bd06502ce44a0ec1e6601b3d5fcb2347d01c3a168ff4aa7e873e21d4878df640a73fabb041d0441840ab6bfa01",
                  "sourceShard": 1,
                  "destinationShard": 1,
                  "blockNonce": 2450607,
                  "blockHash": "044cbf98fc86238a66bda751797e89935676eabc99a29da70926b93ebc457fa7",
                  "notarizedAtSourceInMetaNonce": 2452016,
                  "NotarizedAtSourceInMetaHash": "5dfc7d1bc969d6ef72ec7cc535c105ec6a50340d52e9e67bd4713ae88a87a180",
                  "notarizedAtDestinationInMetaNonce": 2452016,
                  "notarizedAtDestinationInMetaHash": "5dfc7d1bc969d6ef72ec7cc535c105ec6a50340d52e9e67bd4713ae88a87a180",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "a6b5d41dca20072984337124c975b3b1894fcde7839d78b509ac4360c1002ff7",
                  "hyperblockNonce": 2452016,
                  "hyperblockHash": "5dfc7d1bc969d6ef72ec7cc535c105ec6a50340d52e9e67bd4713ae88a87a180",
                  "timestamp": 1709083490,
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                        "identifier": "signalError",
                        "topics": [
                          "lq3O+DrwJ9dElYcbuyqGhxlbHdDQXM6Y6WqYvfMRBGM=",
                          "d3JvbmcgbnVtYmVyIG9mIGFyZ3VtZW50cw=="
                        ],
                        "data": "QDc1NzM2NTcyMjA2NTcyNzI2Zjcy",
                        "additionalData": [
                          "QDc1NzM2NTcyMjA2NTcyNzI2Zjcy"
                        ]
                      },
                      {
                        "address": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
                        "identifier": "internalVMErrors",
                        "topics": [
                          "AAAAAAAAAAAFAH6UefeHERqHcLpMz2gC3xXGhFsJBGM=",
                          "ZGVwbG95SGF0b21TaW1wbGVMZW5kaW5nU3RyYXRlZ3k="
                        ],
                        "data": "CglydW50aW1lLmdvOjgzMCBbZXJyb3Igc2lnbmFsbGVkIGJ5IHNtYXJ0Y29udHJhY3RdIFtkZXBsb3lIYXRvbVNpbXBsZUxlbmRpbmdTdHJhdGVneV0KCXJ1bnRpbWUuZ286ODMwIFtlcnJvciBzaWduYWxsZWQgYnkgc21hcnRjb250cmFjdF0gW2RlcGxveUhhdG9tU2ltcGxlTGVuZGluZ1N0cmF0ZWd5XQoJcnVudGltZS5nbzo4MjcgW3dyb25nIG51bWJlciBvZiBhcmd1bWVudHNd",
                        "additionalData": [
                          "CglydW50aW1lLmdvOjgzMCBbZXJyb3Igc2lnbmFsbGVkIGJ5IHNtYXJ0Y29udHJhY3RdIFtkZXBsb3lIYXRvbVNpbXBsZUxlbmRpbmdTdHJhdGVneV0KCXJ1bnRpbWUuZ286ODMwIFtlcnJvciBzaWduYWxsZWQgYnkgc21hcnRjb250cmFjdF0gW2RlcGxveUhhdG9tU2ltcGxlTGVuZGluZ1N0cmF0ZWd5XQoJcnVudGltZS5nbzo4MjcgW3dyb25nIG51bWJlciBvZiBhcmd1bWVudHNd"
                        ]
                      }
                    ]
                  },
                  "status": "success",
                  "operation": "transfer",
                  "function": "deployHatomSimpleLendingStrategy",
                  "initiallyPaidFee": "6729630000000000",
                  "fee": "6729630000000000",
                  "chainID": "D",
                  "version": 1,
                  "options": 0
                }
              },
              "error": "",
              "code": "successful"
            }
        "#;

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
            .unwrap()
            .data
            .unwrap()
            .transaction;
        TxResponse::from_network_tx(tx_on_network)
    }

    fn get_tx_indirect_issue_fungible_tx() -> TxResponse {
        let data = r#"
        {
          "data": {
            "transaction": {
              "type": "normal",
              "processingTypeOnSource": "SCInvoking",
              "processingTypeOnDestination": "SCInvoking",
              "hash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
              "nonce": 61,
              "round": 173598,
              "epoch": 72,
              "value": "50000000000000000",
              "receiver": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
              "sender": "erd1x39tc3q3nn72ecjnmcz7x0qp09kp97t080x99dgyhx7zh95j0n4szskhlv",
              "gasPrice": 1000000000,
              "gasLimit": 100000000,
              "gasUsed": 100000000,
              "data": "aXNzdWVMcFRva2VuQDAwMDAwMDAwMDAwMDAwMDAwNTAwMTM5ZWQ3YWU0YWEwMzc5MmU2YmNiMzMyMzk0YTQwZmU3NDZlZWZhNDdjZWJANDU0NzRjNDQ0ZDQ1NTg0YzUwQDQ1NDc0YzQ0NGQ0NTU4",
              "signature": "b5049d2906adc1305a6a8d0f42749254ca6259c6996d9a35e7dc7528b3c87b48a421879aff70bc6d81483a7559b75e5dcf9be499dcb7d57aa9f25c79ac2ad40d",
              "sourceShard": 1,
              "destinationShard": 1,
              "blockNonce": 173354,
              "blockHash": "09d85ac264a54e12e7613395211c53fe0ee5a7d3b7111bf5fec1d02794caaacd",
              "notarizedAtSourceInMetaNonce": 173321,
              "NotarizedAtSourceInMetaHash": "64a83759da97fe8305cd4cda4b518f2d41ef0a8f3995d264460821edad45e09e",
              "notarizedAtDestinationInMetaNonce": 173321,
              "notarizedAtDestinationInMetaHash": "64a83759da97fe8305cd4cda4b518f2d41ef0a8f3995d264460821edad45e09e",
              "miniblockType": "TxBlock",
              "miniblockHash": "7f45eee4e35ffc1fbce66b92e4dd2aeae2acb092416aa5aa775b96493256b81d",
              "hyperblockNonce": 173321,
              "hyperblockHash": "64a83759da97fe8305cd4cda4b518f2d41ef0a8f3995d264460821edad45e09e",
              "timestamp": 1695041588,
              "smartContractResults": [
                {
                  "hash": "bce3d0dceb0b3e5c8c5780d7da3755c3f7492d551685d493a73bf66ebd36754b",
                  "nonce": 0,
                  "value": 50000000000000000,
                  "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "sender": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
                  "data": "issue@45474c444d45584c50@45474c444d4558@03e8@12@63616e467265657a65@74727565@63616e57697065@74727565@63616e5061757365@74727565@63616e4d696e74@74727565@63616e4275726e@74727565@63616e4368616e67654f776e6572@74727565@63616e55706772616465@74727565@63616e4164645370656369616c526f6c6573@74727565@65ba30",
                  "prevTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
                  "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
                  "gasLimit": 89624222,
                  "gasPrice": 1000000000,
                  "callType": 1,
                  "originalSender": "erd1x39tc3q3nn72ecjnmcz7x0qp09kp97t080x99dgyhx7zh95j0n4szskhlv",
                  "operation": "transfer",
                  "function": "issue"
                },
                {
                  "hash": "2a452ff652791d79be5f6933fb583cc5503e876893e54b3b51381a92aa2e904d",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "ESDTSetBurnRoleForAll@45474c444d45582d393563366435",
                  "prevTxHash": "bce3d0dceb0b3e5c8c5780d7da3755c3f7492d551685d493a73bf66ebd36754b",
                  "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "logs": {
                    "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                    "events": [
                      {
                        "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "vOPQ3OsLPlyMV4DX2jdVw/dJLVUWhdSTpzv2br02dUs="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "operation": "transfer"
                },
                {
                  "hash": "2c84740ccb3376ea9fa00dab6c6c93fe7a35ee0a1d6dbfa0a1e61064853b0874",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "ESDTTransfer@45474c444d45582d393563366435@03e8@00",
                  "prevTxHash": "bce3d0dceb0b3e5c8c5780d7da3755c3f7492d551685d493a73bf66ebd36754b",
                  "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
                  "gasLimit": 39624222,
                  "gasPrice": 1000000000,
                  "callType": 2,
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                        "identifier": "ESDTTransfer",
                        "topics": [
                          "RUdMRE1FWC05NWM2ZDU=",
                          "",
                          "A+g=",
                          "AAAAAAAAAAAFAO+ux8+3RD51ieGHV10Z68X293CYfOs="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "vOPQ3OsLPlyMV4DX2jdVw/dJLVUWhdSTpzv2br02dUs="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "tokens": [
                    "EGLDMEX-95c6d5"
                  ],
                  "esdtValues": [
                    "1000"
                  ],
                  "operation": "ESDTTransfer",
                  "function": "\u0000"
                },
                {
                  "hash": "c9dfc4de3c3cee319123087a4f5dd03cc051e728ec6070707a63ea977b535227",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
                  "sender": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
                  "data": "\u0000",
                  "prevTxHash": "2c84740ccb3376ea9fa00dab6c6c93fe7a35ee0a1d6dbfa0a1e61064853b0874",
                  "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
                  "gasLimit": 39424222,
                  "gasPrice": 1000000000,
                  "callType": 2,
                  "operation": "transfer",
                  "function": "\u0000"
                },
                {
                  "hash": "609c3a8e1903680fef1f6d9e47527b1b5c1259664b868af600162120ce0b8192",
                  "nonce": 1,
                  "value": 300925400000000,
                  "receiver": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
                  "sender": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
                  "data": "@6f6b",
                  "prevTxHash": "2c84740ccb3376ea9fa00dab6c6c93fe7a35ee0a1d6dbfa0a1e61064853b0874",
                  "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "operation": "transfer",
                  "isRefund": true
                }
              ],
              "logs": {
                "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
                "events": [
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
                    "identifier": "transferValueOnly",
                    "topics": [
                      "AAAAAAAAAAAFAO+ux8+3RD51ieGHV10Z68X293CYfOs=",
                      "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8=",
                      "saK8LsUAAA=="
                    ],
                    "data": null,
                    "additionalData": null
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
                    "identifier": "writeLog",
                    "topics": [
                      "NEq8RBGc/KziU94F4zwBeWwS+W87zFK1BLm8K5aSfOs="
                    ],
                    "data": "QDZmNmI=",
                    "additionalData": null
                  }
                ]
              },
              "status": "success",
              "operation": "transfer",
              "function": "issueLpToken",
              "initiallyPaidFee": "1214335000000000",
              "fee": "1214335000000000",
              "chainID": "D",
              "version": 2,
              "options": 0
            }
          },
          "error": "",
          "code": "successful"
        }
      "#;

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
            .unwrap()
            .data
            .unwrap()
            .transaction;
        TxResponse::from_network_tx(tx_on_network)
    }

    fn get_indirect_issue_non_fungible_tx() -> TxResponse {
        let data = r#"
        {
          "data": {
            "transaction": {
              "type": "normal",
              "processingTypeOnSource": "SCInvoking",
              "processingTypeOnDestination": "SCInvoking",
              "hash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
              "nonce": 16,
              "round": 820170,
              "epoch": 341,
              "value": "50000000000000000",
              "receiver": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
              "sender": "erd162knt53z7m0f9jjms9wxphr3q9d7zu4ky85xs2cc0ekrqz7k4fdq85lkuc",
              "gasPrice": 1000000000,
              "gasLimit": 200000000,
              "gasUsed": 200000000,
              "data": "aXNzdWVUb2tlbkA2NzY1NmU2NTdhNzk3M0A0NzQ1NGU=",
              "signature": "e80d45f4de419799a2bbff1cae1235521c8eef1853ee45b02f95c2da74ce50d241bf75b6ab0c650245562700862ea9759caad40f3e381ac0c4d82cfe56e67c09",
              "sourceShard": 2,
              "destinationShard": 2,
              "blockNonce": 819313,
              "blockHash": "a1db4ef13f07b86678000df9cc78f244d83dcc35ae51de545f333bf616930d39",
              "notarizedAtSourceInMetaNonce": 819396,
              "NotarizedAtSourceInMetaHash": "6d9e511e46d318aa5b77cbfdfde14d2ce8515ce4e954b286f130a6b518ddf26a",
              "notarizedAtDestinationInMetaNonce": 819396,
              "notarizedAtDestinationInMetaHash": "6d9e511e46d318aa5b77cbfdfde14d2ce8515ce4e954b286f130a6b518ddf26a",
              "miniblockType": "TxBlock",
              "miniblockHash": "afdb278522181aeb9b12f08840e6c534e398e6af9c7f757548308e300e7ec4e9",
              "hyperblockNonce": 819396,
              "hyperblockHash": "6d9e511e46d318aa5b77cbfdfde14d2ce8515ce4e954b286f130a6b518ddf26a",
              "timestamp": 1698921020,
              "smartContractResults": [
                {
                  "hash": "6fe0cc002802af1744f394eee4a69224b5e775961d8386e04e7a5b9242f7ff65",
                  "nonce": 0,
                  "value": 50000000000000000,
                  "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "sender": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                  "data": "issueNonFungible@67656e657a7973@47454e@63616e467265657a65@74727565@63616e57697065@74727565@63616e5061757365@74727565@63616e5472616e736665724e4654437265617465526f6c65@74727565@63616e4368616e67654f776e6572@66616c7365@63616e55706772616465@66616c7365@63616e4164645370656369616c526f6c6573@74727565@5e30e4",
                  "prevTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
                  "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
                  "gasLimit": 196098365,
                  "gasPrice": 1000000000,
                  "callType": 1,
                  "originalSender": "erd162knt53z7m0f9jjms9wxphr3q9d7zu4ky85xs2cc0ekrqz7k4fdq85lkuc",
                  "operation": "transfer",
                  "function": "issueNonFungible"
                },
                {
                  "hash": "98afe82512c79f1caaf171bd5919ee469d11ba0c4f725aefcab834278c0f1e58",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "ESDTSetBurnRoleForAll@47454e2d383638353933",
                  "prevTxHash": "6fe0cc002802af1744f394eee4a69224b5e775961d8386e04e7a5b9242f7ff65",
                  "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "logs": {
                    "address": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
                    "events": [
                      {
                        "address": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "b+DMACgCrxdE85Tu5KaSJLXndZYdg4bgTnpbkkL3/2U="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "operation": "transfer"
                },
                {
                  "hash": "83494ad9369738b574a7266cbfb12ce63ccf634950cd6b0ec16107b8fb42f8f6",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "sender": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                  "data": "setSpecialRole@47454e2d383638353933@00000000000000000500de51fa8943c26e6933419f9bb7ceb79b7ff4f7bbaa5a@45534454526f6c654e4654437265617465@5e30e4",
                  "prevTxHash": "112d18ec0364b4700b1bfb3df517c80dba547a53373ece2a9e71acd5266e7b64",
                  "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
                  "gasLimit": 142399698,
                  "gasPrice": 1000000000,
                  "callType": 1,
                  "operation": "transfer",
                  "function": "setSpecialRole"
                },
                {
                  "hash": "112d18ec0364b4700b1bfb3df517c80dba547a53373ece2a9e71acd5266e7b64",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "@00@47454e2d383638353933",
                  "prevTxHash": "6fe0cc002802af1744f394eee4a69224b5e775961d8386e04e7a5b9242f7ff65",
                  "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
                  "gasLimit": 146098365,
                  "gasPrice": 1000000000,
                  "callType": 2,
                  "originalSender": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                        "identifier": "writeLog",
                        "topics": [
                          "AAAAAAAAAAAFAN5R+olDwm5pM0Gfm7fOt5t/9Pe7qlo="
                        ],
                        "data": "QDZmNmI=",
                        "additionalData": null
                      }
                    ]
                  },
                  "operation": "transfer"
                },
                {
                  "hash": "db5d74970374337956fa61fb4fd90057b3f6a82ea3e259b389934b71a1652e5f",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "ESDTSetRole@47454e2d383638353933@45534454526f6c654e4654437265617465",
                  "prevTxHash": "83494ad9369738b574a7266cbfb12ce63ccf634950cd6b0ec16107b8fb42f8f6",
                  "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                        "identifier": "ESDTSetRole",
                        "topics": [
                          "R0VOLTg2ODU5Mw==",
                          "",
                          "",
                          "RVNEVFJvbGVORlRDcmVhdGU="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "g0lK2TaXOLV0pyZsv7Es5jzPY0lQzWsOwWEHuPtC+PY="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "operation": "ESDTSetRole",
                  "function": "ESDTSetRole"
                },
                {
                  "hash": "a6a665f47977a59c4c2baf460281fc938e04ae0f87ac2e78040a14ae27822701",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "@00",
                  "prevTxHash": "83494ad9369738b574a7266cbfb12ce63ccf634950cd6b0ec16107b8fb42f8f6",
                  "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
                  "gasLimit": 92399698,
                  "gasPrice": 1000000000,
                  "callType": 2,
                  "originalSender": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                        "identifier": "writeLog",
                        "topics": [
                          "AAAAAAAAAAAFAN5R+olDwm5pM0Gfm7fOt5t/9Pe7qlo=",
                          "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gOTIzOTk2OTgsIGdhcyB1c2VkID0gMzE0MTg4MA=="
                        ],
                        "data": "QDZmNmI=",
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "g0lK2TaXOLV0pyZsv7Es5jzPY0lQzWsOwWEHuPtC+PY="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "operation": "transfer"
                }
              ],
              "logs": {
                "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                "events": [
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                    "identifier": "transferValueOnly",
                    "topics": [
                      "AAAAAAAAAAAFAN5R+olDwm5pM0Gfm7fOt5t/9Pe7qlo=",
                      "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8=",
                      "saK8LsUAAA=="
                    ],
                    "data": null,
                    "additionalData": null
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                    "identifier": "writeLog",
                    "topics": [
                      "0q010iL23pLKW4FcYNxxAVvhcrYh6GgrGH5sMAvWqlo="
                    ],
                    "data": "QDZmNmI=",
                    "additionalData": null
                  }
                ]
              },
              "status": "success",
              "operation": "transfer",
              "function": "issueToken",
              "initiallyPaidFee": "2097020000000000",
              "fee": "2097020000000000",
              "chainID": "D",
              "version": 1,
              "options": 0
            }
          },
          "error": "",
          "code": "successful"
        }
        "#;

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
            .unwrap()
            .data
            .unwrap()
            .transaction;

        TxResponse::from_network_tx(tx_on_network)
    }

    fn get_indirect_issue_semi_fungible_tx() -> TxResponse {
        let data = r#"
        {
          "data": {
            "transaction": {
              "type": "normal",
              "processingTypeOnSource": "SCInvoking",
              "processingTypeOnDestination": "SCInvoking",
              "hash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
              "nonce": 65,
              "round": 8422527,
              "epoch": 584,
              "value": "50000000000000000",
              "receiver": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
              "sender": "erd1x3g000ew7zzv6kyqhj9jl2wy5g6cc72qahvvxz29zv76jwq6ssvqt0d998",
              "gasPrice": 1000000000,
              "gasLimit": 80000000,
              "gasUsed": 80000000,
              "data": "aXNzdWVUb2tlbkA0NDZmNzA2NTU0NjU3Mzc0QDQ0NGY1MDQ1NTQ0NTUzNTQ=",
              "signature": "0191848976e930996f6c62d4921e732f9b0ada8b41ca3b5b63d6bfd304fd44c2a1e8e6643479618ba4a764a36e87f53882b4f707600d5b7d476f2fdd2bac040e",
              "sourceShard": 0,
              "destinationShard": 0,
              "blockNonce": 8420241,
              "blockHash": "4d302220f6015876c95e7961b770cc67f8ab63c5f0ab69b4d6c2fb15c8bc23bd",
              "notarizedAtSourceInMetaNonce": 8403647,
              "NotarizedAtSourceInMetaHash": "f8b83b6d38fa45dacc167b15c93dd07ee5c40db906de34f26e11e7a24f539e30",
              "notarizedAtDestinationInMetaNonce": 8403647,
              "notarizedAtDestinationInMetaHash": "f8b83b6d38fa45dacc167b15c93dd07ee5c40db906de34f26e11e7a24f539e30",
              "miniblockType": "TxBlock",
              "miniblockHash": "b7b8fc9f3b81d7daae1113cbf73457e16ee31f3a864ef3729a1a21f3a929e112",
              "hyperblockNonce": 8403647,
              "hyperblockHash": "f8b83b6d38fa45dacc167b15c93dd07ee5c40db906de34f26e11e7a24f539e30",
              "timestamp": 1646652762,
              "smartContractResults": [
                {
                  "hash": "9aecf3bd5dd5c706a28d1cc7059ac20db74340f136816f667dbefcc58daa3aba",
                  "nonce": 0,
                  "value": 50000000000000000,
                  "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "sender": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
                  "data": "issueSemiFungible@446f706554657374@444f504554455354@63616e467265657a65@74727565@63616e57697065@74727565@63616e5061757365@74727565@63616e4368616e67654f776e6572@74727565@63616e55706772616465@74727565@63616e4164645370656369616c526f6c6573@74727565@5ca148",
                  "prevTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
                  "originalTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
                  "gasLimit": 75958360,
                  "gasPrice": 1000000000,
                  "callType": 1,
                  "originalSender": "erd1x3g000ew7zzv6kyqhj9jl2wy5g6cc72qahvvxz29zv76jwq6ssvqt0d998",
                  "operation": "transfer",
                  "function": "issueSemiFungible"
                },
                {
                  "hash": "aacfe9088bb9d2d5b3fbe9cab2b2f1c6a7e9cbab2f1a41020e2c819fc9b43570",
                  "nonce": 66,
                  "value": 0,
                  "receiver": "erd1x3g000ew7zzv6kyqhj9jl2wy5g6cc72qahvvxz29zv76jwq6ssvqt0d998",
                  "sender": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
                  "data": "@6f6b",
                  "prevTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
                  "originalTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "operation": "transfer"
                },
                {
                  "hash": "3f6f0f3de9e942884e7e1592823a7db7ce935a3f9d3359d8c1ee98a5645332d8",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "@00@444f5045544553542d373732303063",
                  "prevTxHash": "9aecf3bd5dd5c706a28d1cc7059ac20db74340f136816f667dbefcc58daa3aba",
                  "originalTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
                  "gasLimit": 25958360,
                  "gasPrice": 1000000000,
                  "callType": 2,
                  "originalSender": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "muzzvV3VxwaijRzHBZrCDbdDQPE2gW9mfb78xY2qOro="
                        ],
                        "data": null
                      }
                    ]
                  },
                  "operation": "transfer"
                },
                {
                  "hash": "c6e4f7c5da455009fb4f6967ce8a273a97b826aa617fa798ffd0cf17bde6b97a",
                  "nonce": 1,
                  "value": 225516180000000,
                  "receiver": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
                  "sender": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
                  "data": "@6f6b",
                  "prevTxHash": "3f6f0f3de9e942884e7e1592823a7db7ce935a3f9d3359d8c1ee98a5645332d8",
                  "originalTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "operation": "transfer",
                  "isRefund": true
                }
              ],
              "logs": {
                "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
                "events": [
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
                    "identifier": "transferValueOnly",
                    "topics": [
                      "AAAAAAAAAAAFAH6d74PDz8xLqvowrlOA5lVDBMUghBg=",
                      "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8=",
                      "saK8LsUAAA=="
                    ],
                    "data": null
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
                    "identifier": "writeLog",
                    "topics": [
                      "NFD3vy7whM1YgLyLL6nEojWMeUDt2MMJRRM9qTgahBg="
                    ],
                    "data": "QDZmNmI="
                  }
                ]
              },
              "status": "success",
              "operation": "transfer",
              "function": "issueToken",
              "initiallyPaidFee": "914840000000000",
              "fee": "914840000000000",
              "chainID": "1",
              "version": 1,
              "options": 0
            }
          },
          "error": "",
          "code": "successful"
        }
        "#;

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
            .unwrap()
            .data
            .unwrap()
            .transaction;
        TxResponse::from_network_tx(tx_on_network)
    }

    fn get_indirect_register_meta_esdt_tx() -> TxResponse {
        let data = r#"
        {
          "data": {
            "transaction": {
              "type": "normal",
              "processingTypeOnSource": "SCInvoking",
              "processingTypeOnDestination": "SCInvoking",
              "hash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
              "nonce": 419,
              "round": 1787093,
              "epoch": 744,
              "value": "50000000000000000",
              "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
              "sender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
              "gasPrice": 1000000000,
              "gasLimit": 600000000,
              "gasUsed": 157220928,
              "data": "ZGVwbG95QXNoc3dhcExQQUNTdHJhdGVneUA0MTRjNTAyZDYzNjE2NTYxNjMzNUA0MTU0NTMyZDM0NjMzMDM5MzIzMEAwM2U4QDAzZThAQDNiOWFjYTAwQDAwMDAwMDAwMDAwMDAwMDAwNTAwOTU3MzkwYWVkYTQzMmY1MmE0MTFkNTE5NzRmZTkzZDQwZDI3NzMzZTA0NjNAMDAwMDAwMDAwMDAwMDAwMDA1MDBkMTJjYzczY2JkYTZmMjY1OWM5NTllNWQ1NzU4YWY5MmNhMTM4NDg2NTIzM0AwMDAwMDAwMDAwMDAwMDAwMDUwMDUxZGY3MTc1OGNmMmFjYTViNDZkZWQ4MTU1OGI1NTE1ZGMyOWYzZjM1MjMzQEAwMDAwMDAwMDAwMDAwMDAwMDUwMDdlNGExZGZjNDM3Y2VkNDlkYjlmMTYzNzk4NDE2Yjg0YWMyMWQ0Yzk3Y2ViMDAwMDAwMGM1NzQ1NDc0YzQ0MmQ2MTMyMzg2MzM1MzkwMDAwMDAwMDAwMDAwMDAwMDUwMGE4YmE5ZTY4NjI2YmJjOTkzZmQ3OTVlOGJiNmY0Nzk0M2IyZjVmZmE3Y2ViMDAwMDAwMGE1NTU0NGIyZDMxMzQ2NDM1Mzc2NEAwMDAwMDAwMTAwMDAwMDAwMDAwMDAwMDAwNTAwNTFkZjcxNzU4Y2YyYWNhNWI0NmRlZDgxNTU4YjU1MTVkYzI5ZjNmMzUyMzMwMDAwMDAwYjQyNTU1MzQ0MmQ2NDM0NjMzMDMxMzQwMDAwMDAwMDAwQDAxODZhMEAyNzEw",
              "signature": "4648af0b96eb430e4986b9fb760549742de09c809b46b984e5d995c898d80c25bfc0717c30da34bd89cd3005d98ee895afa39ee588b7b74b4807c63cbeade807",
              "sourceShard": 1,
              "destinationShard": 1,
              "blockNonce": 1785520,
              "blockHash": "8f926a5d79fa84bc69949a21bfbba17447091a8a074ac172fa0b88e4475a1214",
              "notarizedAtSourceInMetaNonce": 1785568,
              "NotarizedAtSourceInMetaHash": "eebd1aa5c3dde083f9c367242c054affedd36bfc95f7bcc1d4e2d90beb5754e9",
              "notarizedAtDestinationInMetaNonce": 1785568,
              "notarizedAtDestinationInMetaHash": "eebd1aa5c3dde083f9c367242c054affedd36bfc95f7bcc1d4e2d90beb5754e9",
              "miniblockType": "TxBlock",
              "miniblockHash": "b85d82db6d69cbc1911b3455d2837eeb3170b391926efa2eacb4d9c8e3c96ee4",
              "hyperblockNonce": 1785568,
              "hyperblockHash": "eebd1aa5c3dde083f9c367242c054affedd36bfc95f7bcc1d4e2d90beb5754e9",
              "timestamp": 1704722558,
              "smartContractResults": [
                {
                  "hash": "ea9a96c079e66249e6b73c0341991dad96ca81f855f2fc4abe0d432be117a882",
                  "nonce": 420,
                  "value": 4427790720000000,
                  "receiver": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
                  "sender": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                  "data": "@6f6b",
                  "prevTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
                  "originalTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "operation": "transfer",
                  "isRefund": true
                },
                {
                  "hash": "6082975132a2c9d8197dfd0f9852b454ad344740eebdbdf93f620b2796ab723b",
                  "nonce": 0,
                  "value": 50000000000000000,
                  "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "sender": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                  "data": "registerMetaESDT@415453417368537761704c5041435661756c74@4156415348@12@63616e467265657a65@66616c7365@63616e57697065@66616c7365@63616e5061757365@66616c7365@63616e5472616e736665724e4654437265617465526f6c65@66616c7365@63616e4368616e67654f776e6572@66616c7365@63616e55706772616465@66616c7365@63616e4164645370656369616c526f6c6573@74727565@9eb30a87c92674ab1469700c0b385b3850e86de80f87dec6cf3213c7e379a646@408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43@03eb4a30",
                  "prevTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
                  "originalTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
                  "gasLimit": 125751600,
                  "gasPrice": 1000000000,
                  "callType": 1,
                  "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
                  "operation": "transfer",
                  "function": "registerMetaESDT"
                },
                {
                  "hash": "290f85d7ec2f7d5797510290358e9e0f76bb880451efaacb0d69280b8d94c67a",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "ESDTSetBurnRoleForAll@41564153482d376438623564",
                  "prevTxHash": "6082975132a2c9d8197dfd0f9852b454ad344740eebdbdf93f620b2796ab723b",
                  "originalTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
                  "logs": {
                    "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                    "events": [
                      {
                        "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "YIKXUTKiydgZff0PmFK0VK00R0Duvb35P2ILJ5arcjs="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "operation": "transfer"
                },
                {
                  "hash": "1aa62a6251edd216bd4e5ae59f7e676d5d2f88597685e0ec0e25ac4434bfccdb",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "@00@41564153482d376438623564@d0644194444642fd16ee156307f6fda0e8f8baf4c496e1a1dc85e027ecc08a4a@9eb30a87c92674ab1469700c0b385b3850e86de80f87dec6cf3213c7e379a646@408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43@00",
                  "prevTxHash": "6082975132a2c9d8197dfd0f9852b454ad344740eebdbdf93f620b2796ab723b",
                  "originalTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
                  "gasLimit": 75751600,
                  "gasPrice": 1000000000,
                  "callType": 2,
                  "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                        "identifier": "writeLog",
                        "topics": [
                          "AAAAAAAAAAAFAH6UefeHERqHcLpMz2gC3xXGhFsJBGM=",
                          "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gNzU3NTE2MDAsIGdhcyB1c2VkID0gNDE3NjA1OQ=="
                        ],
                        "data": "QDZmNmI=",
                        "additionalData": [
                          "QDZmNmI="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "YIKXUTKiydgZff0PmFK0VK00R0Duvb35P2ILJ5arcjs="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "operation": "transfer"
                }
              ],
              "logs": {
                "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                "events": [
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                    "identifier": "transferValueOnly",
                    "topics": [
                      "",
                      "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM="
                    ],
                    "data": "RGVwbG95RnJvbVNvdXJjZQ==",
                    "additionalData": [
                      "RGVwbG95RnJvbVNvdXJjZQ==",
                      "aW5pdA==",
                      "QUxQLWNhZWFjNQ==",
                      "QVRTLTRjMDkyMA==",
                      "A+g=",
                      "A+g=",
                      "",
                      "O5rKAA=="
                    ]
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                    "identifier": "transferValueOnly",
                    "topics": [
                      "",
                      "AAAAAAAAAAAFADJ0SE0vUW6bO5SurLeFIMfK/HtBBGM="
                    ],
                    "data": "RGVwbG95RnJvbVNvdXJjZQ==",
                    "additionalData": [
                      "RGVwbG95RnJvbVNvdXJjZQ==",
                      "aW5pdA==",
                      "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM=",
                      "AAAAAAAAAAAFAJVzkK7aQy9SpBHVGXT+k9QNJ3M+BGM=",
                      "AAAAAAAAAAAFANEsxzy9pvJlnJWeXVdYr5LKE4SGUjM=",
                      "AAAAAAAAAAAFAFHfcXWM8qyltG3tgVWLVRXcKfPzUjM=",
                      "",
                      "AAAAAAAAAAAFAH5KHfxDfO1J258WN5hBa4SsIdTJfOsAAAAMV0VHTEQtYTI4YzU5AAAAAAAAAAAFAKi6nmhia7yZP9eV6LtvR5Q7L1/6fOsAAAAKVVRLLTE0ZDU3ZA==",
                      "AAAAAQAAAAAAAAAABQBR33F1jPKspbRt7YFVi1UV3Cnz81IzAAAAC0JVU0QtZDRjMDE0AAAAAAA=",
                      "AYag",
                      "JxA="
                    ]
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqxf6ysnf029hfkwu546kt0pfqcl90c76pq33s0a320f",
                    "identifier": "transferValueOnly",
                    "topics": [
                      "",
                      "AAAAAAAAAAAFANEsxzy9pvJlnJWeXVdYr5LKE4SGUjM="
                    ],
                    "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                    "additionalData": [
                      "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                      "Z2V0RmFybWluZ1Rva2VuSWQ="
                    ]
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqxf6ysnf029hfkwu546kt0pfqcl90c76pq33s0a320f",
                    "identifier": "transferValueOnly",
                    "topics": [
                      "",
                      "AAAAAAAAAAAFANEsxzy9pvJlnJWeXVdYr5LKE4SGUjM="
                    ],
                    "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                    "additionalData": [
                      "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                      "Z2V0RmFybVRva2VuSWQ="
                    ]
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqxf6ysnf029hfkwu546kt0pfqcl90c76pq33s0a320f",
                    "identifier": "transferValueOnly",
                    "topics": [
                      "",
                      "AAAAAAAAAAAFANEsxzy9pvJlnJWeXVdYr5LKE4SGUjM="
                    ],
                    "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                    "additionalData": [
                      "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                      "Z2V0UmV3YXJkVG9rZW5JZA=="
                    ]
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                    "identifier": "transferValueOnly",
                    "topics": [
                      "saK8LsUAAA==",
                      "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8="
                    ],
                    "data": "QXN5bmNDYWxs",
                    "additionalData": [
                      "QXN5bmNDYWxs",
                      "cmVnaXN0ZXJNZXRhRVNEVA==",
                      "QVRTQXNoU3dhcExQQUNWYXVsdA==",
                      "QVZBU0g=",
                      "Eg==",
                      "Y2FuRnJlZXpl",
                      "ZmFsc2U=",
                      "Y2FuV2lwZQ==",
                      "ZmFsc2U=",
                      "Y2FuUGF1c2U=",
                      "ZmFsc2U=",
                      "Y2FuVHJhbnNmZXJORlRDcmVhdGVSb2xl",
                      "ZmFsc2U=",
                      "Y2FuQ2hhbmdlT3duZXI=",
                      "ZmFsc2U=",
                      "Y2FuVXBncmFkZQ==",
                      "ZmFsc2U=",
                      "Y2FuQWRkU3BlY2lhbFJvbGVz",
                      "dHJ1ZQ=="
                    ]
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqxf6ysnf029hfkwu546kt0pfqcl90c76pq33s0a320f",
                    "identifier": "SCDeploy",
                    "topics": [
                      "AAAAAAAAAAAFADJ0SE0vUW6bO5SurLeFIMfK/HtBBGM=",
                      "AAAAAAAAAAAFAH6UefeHERqHcLpMz2gC3xXGhFsJBGM=",
                      "fvRqbue54Womde/CN2IkRGkrx8tsU+xkLvi3+uwMkhY="
                    ],
                    "data": null,
                    "additionalData": null
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq6qxvpe3csllkk7fdxs3553884344wh2tq33sakulat",
                    "identifier": "SCDeploy",
                    "topics": [
                      "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM=",
                      "AAAAAAAAAAAFAH6UefeHERqHcLpMz2gC3xXGhFsJBGM=",
                      "E3blQfRJfCKLWDr06Od703DSZenIzq8KND+xUjmGY/M="
                    ],
                    "data": null,
                    "additionalData": null
                  }
                ]
              },
              "status": "success",
              "operation": "transfer",
              "function": "deployAshswapLPACStrategy",
              "initiallyPaidFee": "6936045000000000",
              "fee": "2508254280000000",
              "chainID": "D",
              "version": 1,
              "options": 0
            }
          },
          "error": "",
          "code": "successful"
        }
        "#;

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
            .unwrap()
            .data
            .unwrap()
            .transaction;
        TxResponse::from_network_tx(tx_on_network)
    }
}