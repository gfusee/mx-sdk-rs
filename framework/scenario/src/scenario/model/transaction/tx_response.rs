use crate::multiversx_sc::types::Address;
use multiversx_chain_vm::tx_mock::TxResult;
use multiversx_sdk::data::transaction::{
    ApiLogs, ApiSmartContractResult, Events, TransactionOnNetwork,
};

use super::{Log, TxExpect, TxResponseStatus};

const LOG_IDENTIFIER_SC_DEPLOY: &str = "SCDeploy";
const LOG_IDENTIFIER_SIGNAL_ERROR: &str = "signalError";

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";

#[derive(Debug, Default, Clone)]
pub struct TxResponse {
    pub out: Vec<Vec<u8>>,
    pub new_deployed_address: Option<Address>,
    pub tx_error: TxResponseStatus,
    pub logs: Vec<Log>,
    pub gas: u64,
    pub refund: u64,
    pub api_scrs: Vec<ApiSmartContractResult>,
    pub api_logs: Option<ApiLogs>,
}

impl TxResponse {
    /// Creates a scenario "expect" field based on the real response.
    ///
    /// Useful for creating traces that also check the results come out always the same.
    pub fn to_expect(&self) -> TxExpect {
        if self.tx_error.is_success() {
            let mut tx_expect = TxExpect::ok();
            if self.out.is_empty() {
                tx_expect = tx_expect.no_result();
            } else {
                for raw_result in &self.out {
                    let result_hex_string = format!("0x{}", hex::encode(raw_result));
                    tx_expect = tx_expect.result(result_hex_string.as_str());
                }
            }
            tx_expect
        } else {
            TxExpect::err(
                self.tx_error.status,
                format!("str:{}", self.tx_error.message),
            )
        }
    }

    pub fn from_tx_result(tx_result: TxResult) -> Self {
        TxResponse {
            out: tx_result.result_values,
            tx_error: TxResponseStatus {
                status: tx_result.result_status,
                message: tx_result.result_message,
            },
            ..Default::default()
        }
    }

    pub fn from_network_tx(tx: TransactionOnNetwork) -> Self {
        let mut response = Self {
            api_scrs: tx.smart_contract_results.unwrap_or_default(),
            api_logs: tx.logs,
            ..Default::default()
        };

        response.tx_error = response.process_signal_error();
        if !response.tx_error.is_success() {
            return response;
        }

        response.process()
    }

    pub fn from_raw_results(raw_results: Vec<Vec<u8>>) -> Self {
        TxResponse {
            out: raw_results,
            ..Default::default()
        }
    }

    fn process(self) -> Self {
        self.process_out().process_new_deployed_address()
    }

    fn process_out(mut self) -> Self {
        if let Some(first_scr) = self.api_scrs.get(0) {
            self.out = decode_scr_data_or_panic(first_scr.data.as_str());
        }
        self
    }

    fn process_new_deployed_address(mut self) -> Self {
        if let Some(event) = self.find_log(LOG_IDENTIFIER_SC_DEPLOY).cloned() {
            // handle topics
            if let Some(topics) = event.topics.as_ref() {
                if topics.len() != 2 {
                    self.tx_error.message.push_str(
                        format!("expected to have 2 topics, found {} instead", topics.len())
                            .as_str(),
                    );
                }

                let address_raw = base64::decode(topics.get(0).unwrap()).unwrap();
                let address = Address::from_slice(address_raw.as_slice());
                self.new_deployed_address = Some(address);
            } else {
                self.tx_error.message.push_str("missing topics");
            }
        }

        self
    }

    // Finds api logs matching the given log identifier.
    fn find_log(&self, log_identifier: &str) -> Option<&Events> {
        if let Some(logs) = &self.api_logs {
            logs.events
                .iter()
                .find(|event| event.identifier == log_identifier)
        } else {
            None
        }
    }

    fn process_signal_error(&self) -> TxResponseStatus {
        let mut tx_error = TxResponseStatus::default();

        if let Some(event) = self.find_log(LOG_IDENTIFIER_SIGNAL_ERROR) {
            tx_error.status = 4;
            tx_error.message = "signal error: ".to_string();

            if let Some(topics) = event.topics.as_ref() {
                if topics.len() != 2 {
                    tx_error.message.push_str(
                        format!(" expected to have 2 topics, found {} instead", topics.len())
                            .as_str(),
                    );
                }

                let error_raw = base64::decode(topics.get(1).unwrap()).unwrap();
                let error = String::from_utf8(error_raw).unwrap();

                tx_error.message.push_str(&error);
            } else {
                tx_error.message.push_str("missing topics");
            }
        }

        tx_error
    }

    pub fn handle_signal_error_event(&self) -> Result<(), TxResponseStatus> {
        if !self.tx_error.is_success() {
            Err(self.tx_error.clone())
        } else {
            Ok(())
        }
    }

    pub fn new_deployed_address(&self) -> Result<Address, TxResponseStatus> {
        if !self.tx_error.is_success() {
            Err(self.tx_error.clone())
        } else {
            Ok(self.new_deployed_address.clone().unwrap())
        }
    }

    // Returns the token identifier of the newly issued non-fungible token.
    pub fn issue_non_fungible_new_token_identifier(&self) -> Result<String, &'static str> {
        let token_identifier_issue_scr: Option<&ApiSmartContractResult> = self
            .api_scrs
            .iter()
            .find(|scr| scr.sender.to_string() == SYSTEM_SC_BECH32 && scr.data.starts_with("@00@"));

        if token_identifier_issue_scr.is_none() {
            return Err("no token identifier issue SCR found");
        }

        let token_identifier_issue_scr = token_identifier_issue_scr.unwrap();
        let encoded_tid = token_identifier_issue_scr.data.split('@').nth(2);
        if encoded_tid.is_none() {
            return Err("no token identifier found in SCR");
        }

        Ok(String::from_utf8(hex::decode(encoded_tid.unwrap()).unwrap()).unwrap())
    }
}

fn decode_scr_data_or_panic(data: &str) -> Vec<Vec<u8>> {
    let mut split = data.split('@');
    let _ = split.next().expect("SCR data should start with '@'");
    let result_code = split.next().expect("missing result code");
    assert_eq!(result_code, "6f6b", "result code is not 'ok'");

    split
        .map(|encoded_arg| hex::decode(encoded_arg).expect("error hex-decoding result"))
        .collect()
}
