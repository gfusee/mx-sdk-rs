use std::fmt::{Display, Formatter};
use base64::Engine;
use multiversx_sdk::data::transaction::{ApiLogs, ApiSmartContractResult, Events};

#[derive(PartialEq, Debug, Clone)]
pub enum TransactionsOutcomeParserError {
    ErrorInTransaction { data: String, message: String },
    NoEventOfType { r#type: String },
    MoreThanOneEventOfType { r#type: String },
    CannotExtractTokenIdentifierFromEvent,
    CannotDecodeBase64StringToUTF8 { base64: String }
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
            TransactionsOutcomeParserError::CannotExtractTokenIdentifierFromEvent => {
                write!(f, "Cannot extract token identifier from event")
            },
            TransactionsOutcomeParserError::CannotDecodeBase64StringToUTF8 { base64 } => {
                write!(f, "Cannot decode base64 string to utf8: {base64}")
            },
        }
    }
}

pub trait TransactionOutcome {
    fn get_smart_contract_results(self) -> Vec<ApiSmartContractResult>;
    fn get_transaction_logs(self) -> Option<ApiLogs>;

    fn get_transaction_logs_ref(&self) -> &Option<ApiLogs>;

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

fn extract_token_identifier(event: &Events) -> Result<String, TransactionsOutcomeParserError> {
    let Some(topics) = event.topics.as_ref() else {
        return Err(TransactionsOutcomeParserError::CannotExtractTokenIdentifierFromEvent)
    };

    let Some(first_topic) = topics.get(0) else {
        return Err(TransactionsOutcomeParserError::CannotExtractTokenIdentifierFromEvent)
    };

    base64_to_utf8(first_topic)
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

#[cfg(test)]
mod tests {
    use multiversx_sdk::data::address::Address;
    use multiversx_sdk::data::transaction::{ApiLogs, Events, TransactionInfo, TransactionOnNetwork};
    use crate::scenario_model::transaction::tx_outcome::{find_single_event_by_identifier, TransactionOutcome, TransactionsOutcomeParserError};
    use crate::scenario_model::TxResponse;

    #[test]
    fn test_ensure_no_error_with_successful_tx() {
        let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "BuiltInFunctionCall",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                  "nonce": 30,
                  "round": 7639115,
                  "epoch": 6333,
                  "value": "0",
                  "receiver": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                  "sender": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                  "gasPrice": 1000000000,
                  "gasLimit": 25500000,
                  "gasUsed": 15297149,
                  "data": "RVNEVFRyYW5zZmVyQDQ4NTQ0ZDJkNjY2NTMxNjYzNjM5QDBkZTBiNmIzYTc2NDAwMDBANzM3NzYxNzA1NDZmNmI2NTZlNzM0NjY5Nzg2NTY0NDk2ZTcwNzU3NEA1NzQ1NDc0YzQ0MmQ2NDM3NjMzNjYyNjJAMDM3Yzc3OGZjY2U5YzU1Yg==",
                  "signature": "e912fae4b7a9e51ddf316a5e82a0f457d453a62e3c17477f5d6175e1b33c5e92ddb187d65f54cf3131a0603321290279a0456c20778039f2ab09b54e33c60f0d",
                  "sourceShard": 2,
                  "destinationShard": 1,
                  "blockNonce": 7585351,
                  "blockHash": "e456f38f11fec78ed26d5fda068e912739dceedb2e5ce559bf17614b8386c039",
                  "notarizedAtSourceInMetaNonce": 7601495,
                  "NotarizedAtSourceInMetaHash": "e28c6011d4b3f73f3945cae70ff251e675dfea331a70077c5ab3310e3101af17",
                  "notarizedAtDestinationInMetaNonce": 7601499,
                  "notarizedAtDestinationInMetaHash": "333d4266614e981cc1c5654f85ef496038a8cddac46dfc0ad0b7c44c37ab489d",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "13e041f32fde79ebf1abdcfe692e99516f9ec6778dcb917251b440daa7f1210a",
                  "hyperblockNonce": 7601499,
                  "hyperblockHash": "333d4266614e981cc1c5654f85ef496038a8cddac46dfc0ad0b7c44c37ab489d",
                  "timestamp": 1694386290,
                  "smartContractResults": [
                    {
                      "hash": "a23faa3c80bae0b968f007ff0fad3afdec05b4e71d749c3d583dec10c6eb05a2",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                      "sender": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                      "data": "ESDTTransfer@5745474c442d643763366262@03856446ff9a304b",
                      "prevTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "originalTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "logs": {
                        "address": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                        "events": [
                          {
                            "address": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                            "identifier": "ESDTTransfer",
                            "topics": [
                              "V0VHTEQtZDdjNmJi",
                              "",
                              "A4VkRv+aMEs=",
                              "qP29NHPKNFkQzDGmOEIaNRCFYABw8ku13ZyenAuvJOY="
                            ],
                            "data": null
                          },
                          {
                            "address": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                            "identifier": "writeLog",
                            "topics": [
                              "AAAAAAAAAAAFAKVe/p1dXpaw/INtyTPaxf3N3LaNfOs="
                            ],
                            "data": "QDZmNmI="
                          },
                          {
                            "address": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "1AWL08E9sLFIMsfFj+Fj2y9Xn/ZUQ4BYa4on2ItKUHA="
                            ],
                            "data": null
                          }
                        ]
                      },
                      "tokens": [
                        "WEGLD-d7c6bb"
                      ],
                      "esdtValues": [
                        "253719210115084363"
                      ],
                      "operation": "ESDTTransfer"
                    },
                    {
                      "hash": "b7b4d15917fd215399d8e772c3c4e732008baaedc2b8172f71c91708ba7523f0",
                      "nonce": 31,
                      "value": 102028510000000,
                      "receiver": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                      "sender": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                      "data": "@6f6b@0000000c5745474c442d64376336626200000000000000000000000803856446ff9a304b@10",
                      "prevTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "originalTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "logs": {
                        "address": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                        "events": [
                          {
                            "address": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "1AWL08E9sLFIMsfFj+Fj2y9Xn/ZUQ4BYa4on2ItKUHA="
                            ],
                            "data": null
                          }
                        ]
                      },
                      "operation": "transfer",
                      "isRefund": true
                    },
                    {
                      "hash": "05a766ca05d2053d1c0fbeb1797116474a06c86402a3bfd6c132c9a24cfa1bb0",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                      "sender": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                      "data": "swapTokensFixedInput@5745474c442d643763366262@037c778fcce9c55b",
                      "prevTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "originalTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "gasLimit": 25050500,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "operation": "transfer",
                      "function": "swapTokensFixedInput"
                    },
                    {
                      "hash": "4e639c80822d5d7780c8326d683fa9cd6d59649d14122dfabc5a96dda36da527",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy",
                      "sender": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                      "data": "ESDTTransfer@5745474c442d643763366262@e7730d1ef1b0@737761704e6f466565416e64466f7277617264@4d45582d646332383963@0000000000000000000000000000000000000000000000000000000000000000",
                      "prevTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "originalTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "tokens": [
                        "WEGLD-d7c6bb"
                      ],
                      "esdtValues": [
                        "254481327387056"
                      ],
                      "operation": "ESDTTransfer",
                      "function": "swapNoFeeAndForward"
                    }
                  ],
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                    "events": [
                      {
                        "address": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                        "identifier": "ESDTTransfer",
                        "topics": [
                          "SFRNLWZlMWY2OQ==",
                          "",
                          "DeC2s6dkAAA=",
                          "AAAAAAAAAAAFAKVe/p1dXpaw/INtyTPaxf3N3LaNfOs="
                        ],
                        "data": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                        "identifier": "ESDTTransfer",
                        "topics": [
                          "V0VHTEQtZDdjNmJi",
                          "",
                          "53MNHvGw",
                          "AAAAAAAAAAAFAOcoOHa5zr9eiFpjeVvIJxVDpaz7fOs="
                        ],
                        "data": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy",
                        "identifier": "ESDTLocalBurn",
                        "topics": [
                          "TUVYLWRjMjg5Yw==",
                          "",
                          "AuMDPq1jy03x"
                        ],
                        "data": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy",
                        "identifier": "swapNoFeeAndForward",
                        "topics": [
                          "c3dhcF9ub19mZWVfYW5kX2ZvcndhcmQ=",
                          "TUVYLWRjMjg5Yw==",
                          "AAAAAAAAAAAFAKVe/p1dXpaw/INtyTPaxf3N3LaNfOs=",
                          "GL0="
                        ],
                        "data": "AAAAAAAAAAAFAKVe/p1dXpaw/INtyTPaxf3N3LaNfOsAAAAMV0VHTEQtZDdjNmJiAAAABudzDR7xsAAAAApNRVgtZGMyODljAAAACQLjAz6tY8tN8QAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABzvkcAAAAAAAAYvQAAAABk/khy"
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                        "identifier": "ESDTTransfer",
                        "topics": [
                          "V0VHTEQtZDdjNmJi",
                          "",
                          "A4VkRv+aMEs=",
                          "qP29NHPKNFkQzDGmOEIaNRCFYABw8ku13ZyenAuvJOY="
                        ],
                        "data": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                        "identifier": "swapTokensFixedInput",
                        "topics": [
                          "c3dhcA==",
                          "SFRNLWZlMWY2OQ==",
                          "V0VHTEQtZDdjNmJi",
                          "qP29NHPKNFkQzDGmOEIaNRCFYABw8ku13ZyenAuvJOY=",
                          "GL0="
                        ],
                        "data": "qP29NHPKNFkQzDGmOEIaNRCFYABw8ku13ZyenAuvJOYAAAAKSFRNLWZlMWY2OQAAAAgN4Lazp2QAAAAAAAxXRUdMRC1kN2M2YmIAAAAIA4VkRv+aMEsAAAAHA41+pMaAAAAAAAoofxtJRPkr8X9kAAAACgpOPCsHUu261HUAAAAAAHO+RwAAAAAAABi9AAAAAGT+SHI="
                      }
                    ]
                  },
                  "status": "success",
                  "tokens": [
                    "HTM-fe1f69"
                  ],
                  "esdtValues": [
                    "1000000000000000000"
                  ],
                  "operation": "ESDTTransfer",
                  "function": "swapTokensFixedInput",
                  "initiallyPaidFee": "502005000000000",
                  "fee": "399976490000000",
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
        let tx_response = TxResponse::from_network_tx(tx_on_network);

        let ensure_no_error_result = tx_response.ensure_no_error();

        assert_eq!(Ok(()), ensure_no_error_result)
    }

    #[test]
    fn test_ensure_no_error_with_failed_tx() {
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
        let tx_response = TxResponse::from_network_tx(tx_on_network);

        let ensure_no_error_result = tx_response.ensure_no_error();

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
}