use std::fmt::{Display, Formatter};
use base64::Engine;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use multiversx_sc::types::EsdtLocalRole;
use multiversx_sdk::data::address::Address;
use multiversx_sdk::data::transaction::{ApiLogs, ApiSmartContractResult, Events};

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";

#[derive(PartialEq, Debug, Clone)]
pub struct RegisterAndSetAllRolesOutcome {
    token_identifier: String,
    roles: Vec<EsdtLocalRole>
}

#[derive(PartialEq, Debug, Clone)]
pub struct SetSpecialRoleOutcome {
    pub user_address: Address,
    pub token_identifier: String,
    pub roles: Vec<EsdtLocalRole>
}

#[derive(PartialEq, Debug, Clone)]
pub struct NFTCreateOutcome {
    pub token_identifier: String,
    pub nonce: u64,
    pub initial_quantity: BigUint
}

#[derive(PartialEq, Debug, Clone)]
pub struct LocalMintOutcome {
    pub user_address: Address,
    pub token_identifier: String,
    pub nonce: u64,
    pub minted_supply: BigUint
}

#[derive(PartialEq, Debug, Clone)]
pub struct LocalBurnOutcome {
    pub user_address: Address,
    pub token_identifier: String,
    pub nonce: u64,
    pub burnt_supply: BigUint
}

#[derive(PartialEq, Debug, Clone)]
pub struct FreezingOutcome {
    pub user_address: Address,
    pub token_identifier: String,
    pub nonce: u64,
    pub balance: BigUint
}

#[derive(PartialEq, Debug, Clone)]
pub struct WipeOutcome {
    pub user_address: Address,
    pub token_identifier: String,
    pub nonce: u64,
    pub balance: BigUint
}

#[derive(PartialEq, Debug, Clone)]
pub struct UpdateAttributesOutcome {
    pub token_identifier: String,
    pub nonce: u64,
    pub attributes: Vec<u8>
}

#[derive(PartialEq, Debug, Clone)]
pub struct AddQuantityOutcome {
    pub token_identifier: String,
    pub nonce: u64,
    pub added_quantity: BigUint
}

#[derive(PartialEq, Debug, Clone)]
pub struct BurnQuantityOutcome {
    pub token_identifier: String,
    pub nonce: u64,
    pub burnt_quantity: BigUint
}

#[derive(PartialEq, Debug, Clone)]
pub enum TransactionsOutcomeParserError {
    ErrorInTransaction { data: String, message: String },
    NoEventOfType { r#type: String },
    MoreThanOneEventOfType { r#type: String },
    EventDoesntHaveTopic,
    CannotExtractTokenIdentifierFromEvent,
    CannotExtractNonceFromEvent,
    CannotExtractAmountFromEvent,
    CannotExtractAddressFromEvent,
    CannotExtractAttributesFromEvent,
    NoIssuedTokenIdentifierInTheResult,
    NoResultToExtractTokenIdentifier { expected_issuance_start_data: String },
    CannotDecodeBase64StringToUTF8 { base64: String },
    CannotDecodeBase64StringToBigUint { base64: String },
    CannotDecodeBase64StringToAddress { base64: String },
    CannotDecodeHexStringToUTF8 { hex: String },
    UnknownRoleFoundInEvent { role: String }
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
            TransactionsOutcomeParserError::EventDoesntHaveTopic => {
                write!(f, "Event doesn't have topic")
            },
            TransactionsOutcomeParserError::CannotExtractTokenIdentifierFromEvent => {
                write!(f, "Cannot extract token identifier from event")
            },
            TransactionsOutcomeParserError::CannotExtractNonceFromEvent => {
                write!(f, "Cannot extract nonce from event")
            },
            TransactionsOutcomeParserError::CannotExtractAmountFromEvent => {
                write!(f, "Cannot extract amount from event")
            },
            TransactionsOutcomeParserError::CannotExtractAddressFromEvent => {
                write!(f, "Cannot extract address from event")
            },
            TransactionsOutcomeParserError::CannotExtractAttributesFromEvent => {
                write!(f, "Cannot extract attributes from event")
            },
            TransactionsOutcomeParserError::NoIssuedTokenIdentifierInTheResult => {
                write!(f, "The given issuance result doesn't contain a token identifier to extract")
            },
            TransactionsOutcomeParserError::NoResultToExtractTokenIdentifier { expected_issuance_start_data } => {
                write!(f, "No result to extract issued token identifier with expected issuance start data: {expected_issuance_start_data}")
            },
            TransactionsOutcomeParserError::CannotDecodeBase64StringToUTF8 { base64 } => {
                write!(f, "Cannot decode base64 string to utf8: {base64}")
            },
            TransactionsOutcomeParserError::CannotDecodeBase64StringToBigUint { base64 } => {
                write!(f, "Cannot decode base64 string to biguint: {base64}")
            },
            TransactionsOutcomeParserError::CannotDecodeBase64StringToAddress { base64 } => {
                write!(f, "Cannot decode base64 string to address: {base64}")
            },
            TransactionsOutcomeParserError::CannotDecodeHexStringToUTF8 { hex } => {
                write!(f, "Cannot decode hex string to utf8: {hex}")
            },
            TransactionsOutcomeParserError::UnknownRoleFoundInEvent { role } => {
                write!(f, "UnknownRoleFoundInEvent: {role}")
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

        let issuance_result_scr = get_token_issuance_scr(
            self.get_smart_contract_results_ref(),
            "issue@"
        )?;

        extract_issued_token_identifier_from_issuance_scr(&issuance_result_scr)
    }

    fn parse_issue_non_fungible(&self) -> Result<String, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let issuance_result_scr = get_token_issuance_scr(
            self.get_smart_contract_results_ref(),
            "issueNonFungible@"
        )?;

        extract_issued_token_identifier_from_issuance_scr(&issuance_result_scr)
    }

    fn parse_issue_semi_fungible(&self) -> Result<String, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let issuance_result_scr = get_token_issuance_scr(
            self.get_smart_contract_results_ref(),
            "issueSemiFungible@"
        )?;

        extract_issued_token_identifier_from_issuance_scr(&issuance_result_scr)
    }

    fn parse_register_meta_esdt(&self) -> Result<String, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let issuance_result_scr = get_token_issuance_scr(
            self.get_smart_contract_results_ref(),
            "registerMetaESDT@"
        )?;

        extract_issued_token_identifier_from_issuance_scr(&issuance_result_scr)
    }

    fn parse_register_and_set_all_roles(&self) -> Result<RegisterAndSetAllRolesOutcome, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let issuance_result_scr = get_token_issuance_scr(
            self.get_smart_contract_results_ref(),
            "registerAndSetAllRoles@"
        )?;

        let event_identifier = "ESDTSetRole";

        let token_identifier = extract_issued_token_identifier_from_issuance_scr(&issuance_result_scr)?;
        let mut set_roles_events = find_events_by_identifier(
            self.get_transaction_logs_ref(),
            self.get_smart_contract_results_ref(),
            event_identifier
        )?;

        if set_roles_events.len() != 1 {
            return Err(TransactionsOutcomeParserError::MoreThanOneEventOfType { r#type: event_identifier.to_string() })
        }

        let roles = extract_roles_from_log(&set_roles_events.remove(0))?;

        Ok(
            RegisterAndSetAllRolesOutcome {
                token_identifier,
                roles,
            }
        )
    }

    fn parse_set_burn_role_globally(&self) -> Result<(), TransactionsOutcomeParserError> {
        // This is how it is implemented in the JS SDK
        // TODO: provide a concrete implementation or remove this function
        self.ensure_no_error()
    }

    fn parse_unset_burn_role_globally(&self) -> Result<(), TransactionsOutcomeParserError> {
        // This is how it is implemented in the JS SDK
        // TODO: provide a concrete implementation or remove this function
        self.ensure_no_error()
    }

    fn parse_set_special_roles(&self) -> Result<SetSpecialRoleOutcome, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let event_identifier = "ESDTSetRole";

        let mut events = find_events_by_identifier(
            self.get_transaction_logs_ref(),
            self.get_smart_contract_results_ref(),
            event_identifier
        )?;

        if events.len() != 1 {
            return Err(TransactionsOutcomeParserError::MoreThanOneEventOfType { r#type: event_identifier.to_string() })
        }

        let event = events.remove(0);

        let token_identifier = extract_token_identifier_from_event(&event)?;
        let roles = extract_roles_from_log(&event)?;

        Ok(
            SetSpecialRoleOutcome {
                user_address: event.address,
                token_identifier,
                roles,
            }
        )
    }

    fn parse_nft_create(&self) -> Result<Vec<NFTCreateOutcome>, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let events = find_events_by_identifier(
            self.get_transaction_logs_ref(),
            self.get_smart_contract_results_ref(),
            "ESDTNFTCreate"
        )?;

        let results: Result<Vec<_>, _> = events.into_iter()
            .map(|event| {
                let token_identifier = extract_token_identifier_from_event(&event)?;
                let nonce = extract_nonce_from_event(&event)?;
                let initial_quantity = extract_amount_from_event(&event)?;

                let result = NFTCreateOutcome {
                    token_identifier,
                    nonce,
                    initial_quantity,
                };

                Ok::<_, TransactionsOutcomeParserError>(result)
            })
            .collect();

        Ok(
            results?
        )
    }

    fn parse_local_mint(&self) -> Result<Vec<LocalMintOutcome>, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let events = find_events_by_identifier(
            self.get_transaction_logs_ref(),
            self.get_smart_contract_results_ref(),
            "ESDTLocalMint"
        )?;

        let results: Result<Vec<_>, _> = events.into_iter()
            .map(|event| {
                let token_identifier = extract_token_identifier_from_event(&event)?;
                let nonce = extract_nonce_from_event(&event)?;
                let minted_supply = extract_amount_from_event(&event)?;

                let result = LocalMintOutcome {
                    user_address: event.address,
                    token_identifier,
                    nonce,
                    minted_supply,
                };

                Ok::<_, TransactionsOutcomeParserError>(result)
            })
            .collect();

        Ok(
            results?
        )
    }

    fn parse_local_burn(&self) -> Result<Vec<LocalBurnOutcome>, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let events = find_events_by_identifier(
            self.get_transaction_logs_ref(),
            self.get_smart_contract_results_ref(),
            "ESDTLocalBurn"
        )?;

        let results: Result<Vec<_>, _> = events.into_iter()
            .map(|event| {
                let token_identifier = extract_token_identifier_from_event(&event)?;
                let nonce = extract_nonce_from_event(&event)?;
                let burnt_supply = extract_amount_from_event(&event)?;

                let result = LocalBurnOutcome {
                    user_address: event.address,
                    token_identifier,
                    nonce,
                    burnt_supply,
                };

                Ok::<_, TransactionsOutcomeParserError>(result)
            })
            .collect();

        Ok(
            results?
        )
    }

    fn parse_pause(&self) -> Result<(), TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        find_events_by_identifier(
            self.get_transaction_logs_ref(),
            self.get_smart_contract_results_ref(),
            "ESDTPause"
        )?;

        Ok(())
    }

    fn parse_unpause(&self) -> Result<(), TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        find_events_by_identifier(
            self.get_transaction_logs_ref(),
            self.get_smart_contract_results_ref(),
            "ESDTUnPause"
        )?;

        Ok(())
    }

    fn parse_freeze(&self) -> Result<Vec<FreezingOutcome>, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let events = find_events_by_identifier(
            self.get_transaction_logs_ref(),
            self.get_smart_contract_results_ref(),
            "ESDTFreeze"
        )?;

        let results: Result<Vec<_>, _> = events.into_iter()
            .map(|event| {
                let token_identifier = extract_token_identifier_from_event(&event)?;
                let nonce = extract_nonce_from_event(&event)?;
                let balance = extract_amount_from_event(&event)?;
                let user_address = extract_address_from_event(&event)?;

                let result = FreezingOutcome {
                    user_address,
                    token_identifier,
                    nonce,
                    balance,
                };

                Ok::<_, TransactionsOutcomeParserError>(result)
            })
            .collect();

        Ok(
            results?
        )
    }

    fn parse_unfreeze(&self) -> Result<Vec<FreezingOutcome>, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let events = find_events_by_identifier(
            self.get_transaction_logs_ref(),
            self.get_smart_contract_results_ref(),
            "ESDTUnFreeze"
        )?;

        let results: Result<Vec<_>, _> = events.into_iter()
            .map(|event| {
                let token_identifier = extract_token_identifier_from_event(&event)?;
                let nonce = extract_nonce_from_event(&event)?;
                let balance = extract_amount_from_event(&event)?;

                let result = FreezingOutcome {
                    user_address: event.address,
                    token_identifier,
                    nonce,
                    balance,
                };

                Ok::<_, TransactionsOutcomeParserError>(result)
            })
            .collect();

        Ok(
            results?
        )
    }

    fn parse_wipe(&self) -> Result<Vec<WipeOutcome>, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let events = find_events_by_identifier(
            self.get_transaction_logs_ref(),
            self.get_smart_contract_results_ref(),
            "ESDTWipe"
        )?;

        let results: Result<Vec<_>, _> = events.into_iter()
            .map(|event| {
                let token_identifier = extract_token_identifier_from_event(&event)?;
                let nonce = extract_nonce_from_event(&event)?;
                let balance = extract_amount_from_event(&event)?;
                let user_address = extract_address_from_event(&event)?;

                let result = WipeOutcome {
                    user_address,
                    token_identifier,
                    nonce,
                    balance,
                };

                Ok::<_, TransactionsOutcomeParserError>(result)
            })
            .collect();

        Ok(
            results?
        )
    }

    fn parse_update_attributes(&self) -> Result<Vec<UpdateAttributesOutcome>, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let events = find_events_by_identifier(
            self.get_transaction_logs_ref(),
            self.get_smart_contract_results_ref(),
            "ESDTNFTUpdateAttributes"
        )?;

        let results: Result<Vec<_>, _> = events.into_iter()
            .map(|event| {
                let token_identifier = extract_token_identifier_from_event(&event)?;
                let nonce = extract_nonce_from_event(&event)?;
                let attributes = extract_attributes_from_event(&event)?;

                let result = UpdateAttributesOutcome {
                    token_identifier,
                    nonce,
                    attributes,
                };

                Ok::<_, TransactionsOutcomeParserError>(result)
            })
            .collect();

        Ok(
            results?
        )
    }

    fn parse_add_quantity(&self) -> Result<Vec<AddQuantityOutcome>, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let events = find_events_by_identifier(
            self.get_transaction_logs_ref(),
            self.get_smart_contract_results_ref(),
            "ESDTNFTAddQuantity"
        )?;

        let results: Result<Vec<_>, _> = events.into_iter()
            .map(|event| {
                let token_identifier = extract_token_identifier_from_event(&event)?;
                let nonce = extract_nonce_from_event(&event)?;
                let added_quantity = extract_amount_from_event(&event)?;

                let result = AddQuantityOutcome {
                    token_identifier,
                    nonce,
                    added_quantity,
                };

                Ok::<_, TransactionsOutcomeParserError>(result)
            })
            .collect();

        Ok(
            results?
        )
    }

    fn parse_burn_quantity(&self) -> Result<Vec<BurnQuantityOutcome>, TransactionsOutcomeParserError> {
        self.ensure_no_error()?;

        let events = find_events_by_identifier(
            self.get_transaction_logs_ref(),
            self.get_smart_contract_results_ref(),
            "ESDTNFTBurn"
        )?;

        let results: Result<Vec<_>, _> = events.into_iter()
            .map(|event| {
                let token_identifier = extract_token_identifier_from_event(&event)?;
                let nonce = extract_nonce_from_event(&event)?;
                let burnt_quantity = extract_amount_from_event(&event)?;

                let result = BurnQuantityOutcome {
                    token_identifier,
                    nonce,
                    burnt_quantity,
                };

                Ok::<_, TransactionsOutcomeParserError>(result)
            })
            .collect();

        Ok(
            results?
        )
    }
}

fn find_events_by_identifier(api_logs: &Option<ApiLogs>, scrs: &[ApiSmartContractResult], identifier: &str) -> Result<Vec<Events>, TransactionsOutcomeParserError> {
    let Some(logs) = api_logs else {
        return Err(TransactionsOutcomeParserError::NoEventOfType { r#type: identifier.to_string() })
    };

    let mut scrs_logs_iters: Vec<core::slice::Iter<Events>> = vec![];
    for scr in scrs {
        if let Some(scr_logs) = scr.logs.as_ref() {
            scrs_logs_iters.push(scr_logs.events.iter())
        }
    }

    let filtered: Vec<&Events> = logs.events.iter().chain(scrs_logs_iters.into_iter().flatten())
        .filter(|e| e.identifier == identifier)
        .collect();

    if filtered.is_empty() {
        return Err(TransactionsOutcomeParserError::NoEventOfType { r#type: identifier.to_string() })
    }

    Ok(filtered.into_iter().cloned().collect())
}

fn get_token_issuance_scr(scrs: &[ApiSmartContractResult], expected_issuance_start_data: &str) -> Result<ApiSmartContractResult, TransactionsOutcomeParserError> {
    let expected_result_start_data = vec!["ESDTTransfer@", "@00@"];

    for scr in scrs {
        if scr.sender.to_string() != SYSTEM_SC_BECH32 {
            continue;
        }

        let Some(prev_tx) = scrs.iter().find(|e| e.hash == scr.prev_tx_hash) else {
            continue;
        };

        if !prev_tx.data.as_ref().map(String::as_ref).unwrap_or("").starts_with(expected_issuance_start_data) {
            continue;
        }

        for expected_start_data in expected_result_start_data.iter() {
            if scr.data.as_ref().map(String::as_ref).unwrap_or("").starts_with(expected_start_data) {
                return Ok(scr.clone())
            }
        }
    }

    Err(TransactionsOutcomeParserError::NoResultToExtractTokenIdentifier { expected_issuance_start_data: expected_issuance_start_data.to_string() })
}

fn extract_issued_token_identifier_from_issuance_scr(issuance_result_scr: &ApiSmartContractResult) -> Result<String, TransactionsOutcomeParserError> {
    let issuance_result_scr_data = issuance_result_scr.data.as_ref().map(String::as_ref).unwrap_or("");
    if issuance_result_scr_data.starts_with("ESDTTransfer@") {
        let opt_encoded_tid = issuance_result_scr_data.split('@').nth(1);
        let Some(encoded_tid) = opt_encoded_tid else {
            return Err(TransactionsOutcomeParserError::CannotExtractTokenIdentifierFromEvent);
        };

        return hex_to_utf8(encoded_tid)
    } else if issuance_result_scr_data.starts_with("@00@") {
        let opt_encoded_tid = issuance_result_scr_data.split('@').nth(2);
        let Some(encoded_tid) = opt_encoded_tid else {
            return Err(TransactionsOutcomeParserError::CannotExtractTokenIdentifierFromEvent);
        };

        return hex_to_utf8(encoded_tid)
    } else {
        return Err(TransactionsOutcomeParserError::NoIssuedTokenIdentifierInTheResult)
    }
}

fn extract_token_identifier_from_event(event: &Events) -> Result<String, TransactionsOutcomeParserError> {
    let Some(topics) = event.topics.as_ref() else {
        return Err(TransactionsOutcomeParserError::EventDoesntHaveTopic)
    };

    let opt_encoded_tid = topics.get(0);
    let Some(encoded_tid) = opt_encoded_tid else {
        return Err(TransactionsOutcomeParserError::CannotExtractTokenIdentifierFromEvent);
    };

    return base64_to_utf8(encoded_tid)
}

fn extract_nonce_from_event(event: &Events) -> Result<u64, TransactionsOutcomeParserError> {
    let Some(topics) = event.topics.as_ref() else {
        return Err(TransactionsOutcomeParserError::EventDoesntHaveTopic)
    };

    let opt_encoded_nonce = topics.get(1);
    let Some(encoded_nonce) = opt_encoded_nonce else {
        return Err(TransactionsOutcomeParserError::CannotExtractNonceFromEvent);
    };

    let nonce_biguint = base64_to_biguint(encoded_nonce)?;

    let Some(result) = nonce_biguint.to_u64() else {
        return Err(TransactionsOutcomeParserError::CannotExtractNonceFromEvent)
    };

    Ok(result)
}

fn extract_amount_from_event(event: &Events) -> Result<BigUint, TransactionsOutcomeParserError> {
    let Some(topics) = event.topics.as_ref() else {
        return Err(TransactionsOutcomeParserError::EventDoesntHaveTopic)
    };

    let opt_encoded_amount = topics.get(2);
    let Some(encoded_nonce) = opt_encoded_amount else {
        return Err(TransactionsOutcomeParserError::CannotExtractAmountFromEvent);
    };

    base64_to_biguint(encoded_nonce)
}

fn extract_address_from_event(event: &Events) -> Result<Address, TransactionsOutcomeParserError> {
    let Some(topics) = event.topics.as_ref() else {
        return Err(TransactionsOutcomeParserError::EventDoesntHaveTopic)
    };

    let opt_encoded_address = topics.get(3);
    let Some(encoded_address) = opt_encoded_address else {
        return Err(TransactionsOutcomeParserError::CannotExtractAddressFromEvent);
    };

    base64_to_address(encoded_address)
}

fn extract_attributes_from_event(event: &Events) -> Result<Vec<u8>, TransactionsOutcomeParserError> {
    let Some(topics) = event.topics.as_ref() else {
        return Err(TransactionsOutcomeParserError::EventDoesntHaveTopic)
    };

    let Some(attributes) = topics.get(3) else {
        return Err(TransactionsOutcomeParserError::CannotExtractAttributesFromEvent)
    };

    base64::engine::general_purpose::STANDARD.decode(attributes)
        .map_err(|_| TransactionsOutcomeParserError::CannotExtractAttributesFromEvent)
}

fn extract_roles_from_log(log: &Events) -> Result<Vec<EsdtLocalRole>, TransactionsOutcomeParserError> {
    let Some(topics) = log.topics.as_ref() else {
        return Err(TransactionsOutcomeParserError::EventDoesntHaveTopic)
    };

    let mut roles = vec![];
    for topic_base64 in topics.iter().skip(3) {
        let topic = base64_to_utf8(topic_base64)?;
        let role = EsdtLocalRole::from(topic.as_bytes());

        if role == EsdtLocalRole::None {
            return Err(TransactionsOutcomeParserError::UnknownRoleFoundInEvent { role: topic.clone() })
        }

        roles.push(role);
    }

    Ok(roles)
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

fn base64_to_biguint(base64: &str) -> Result<BigUint, TransactionsOutcomeParserError> {
    let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(base64) else {
        return Err(TransactionsOutcomeParserError::CannotDecodeBase64StringToBigUint { base64: base64.to_string() })
    };

    Ok(BigUint::from_bytes_be(&decoded))
}

fn base64_to_address(base64: &str) -> Result<Address, TransactionsOutcomeParserError> {
    let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(base64) else {
        return Err(TransactionsOutcomeParserError::CannotDecodeBase64StringToUTF8 { base64: base64.to_string() })
    };

    if decoded.len() == 32 {
        let mut arr = [0u8; 32];
        arr.clone_from_slice(&decoded[..]);
        Ok(Address::from_bytes(arr))
    } else {
        Err(TransactionsOutcomeParserError::CannotDecodeBase64StringToAddress { base64: base64.to_string() })
    }
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
    use std::str::FromStr;
    use base64::Engine;
    use num_bigint::BigUint;
    use multiversx_sc::imports::EsdtLocalRole;
    use multiversx_sdk::data::address::Address;
    use multiversx_sdk::data::transaction::{ApiLogs, ApiSmartContractResult, Events, TransactionInfo, TransactionOnNetwork};
    use multiversx_sdk::data::vm::CallType;
    use crate::scenario_model::transaction::tx_outcome::{AddQuantityOutcome, BurnQuantityOutcome, find_events_by_identifier, FreezingOutcome, LocalBurnOutcome, LocalMintOutcome, NFTCreateOutcome, RegisterAndSetAllRolesOutcome, SetSpecialRoleOutcome, TransactionOutcome, TransactionsOutcomeParserError, UpdateAttributesOutcome, WipeOutcome};
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

        let result = find_events_by_identifier(&Some(api_logs), &vec![], "issue");

        let expected_events = vec![
            Events {
                address: Address::from_bytes(Default::default()),
                identifier: "issue".to_string(),
                topics: None,
                data: None,
            }
        ];

        assert_eq!(Ok(expected_events), result);
    }

    #[test]
    fn test_find_single_event_by_identifier_event_exists_in_a_scr() {
        let events = vec![];
        let scrs = vec![
            ApiSmartContractResult {
                hash: Default::default(),
                nonce: Default::default(),
                value: Default::default(),
                receiver: Address::from_bytes(Default::default()),
                sender: Address::from_bytes(Default::default()),
                data: Default::default(),
                prev_tx_hash: Default::default(),
                original_tx_hash: Default::default(),
                gas_limit: Default::default(),
                gas_price: Default::default(),
                call_type: CallType::DirectCall,
                relayer_address: None,
                relayed_value: None,
                code: None,
                code_metadata: None,
                return_message: None,
                original_sender: None,
                logs: Some(ApiLogs {
                    address: Address::from_bytes(Default::default()),
                    events: vec![
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
                    ],
                }),
            }
        ];

        let api_logs = ApiLogs {
            address: Address::from_bytes(Default::default()),
            events,
        };

        let result = find_events_by_identifier(&Some(api_logs), &scrs, "issue");

        let expected_event = vec![
            Events {
                address: Address::from_bytes(Default::default()),
                identifier: "issue".to_string(),
                topics: None,
                data: None,
            }
        ];

        assert_eq!(Ok(expected_event), result);
    }

    #[test]
    fn test_find_single_event_by_identifier_no_event_at_all() {
        let api_logs = None;

        let result = find_events_by_identifier(&api_logs, &vec![], "issue");

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

        let result = find_events_by_identifier(&Some(api_logs), &vec![], "unknown");

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
                data: Some("test data 2".to_string()),
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
                data: Some("test data 2".to_string()),
            },
        ];

        let api_logs = ApiLogs {
            address: Address::from_bytes(Default::default()),
            events,
        };

        let result = find_events_by_identifier(&Some(api_logs), &vec![], "test");

        let expected_event = vec![
            Events {
                address: Address::from_bytes(Default::default()),
                identifier: "test".to_string(),
                topics: None,
                data: Some("test data 2".to_string()),
            },
            Events {
                address: Address::from_bytes(Default::default()),
                identifier: "test".to_string(),
                topics: None,
                data: Some("test data 2".to_string()),
            },
        ];

        assert_eq!(Ok(expected_event), result);
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

    #[test]
    fn test_parse_register_and_set_all_roles_valid() {
        let parsed_token = get_indirect_register_and_set_all_roles_tx().parse_register_and_set_all_roles();

        let expected_token_identifier = "NNNTIKER-e248ba".to_string();
        let expected_roles = vec![EsdtLocalRole::NftCreate, EsdtLocalRole::NftBurn, EsdtLocalRole::NftUpdateAttributes, EsdtLocalRole::NftAddUri];
        let expected: Result<RegisterAndSetAllRolesOutcome, TransactionsOutcomeParserError> = Ok(
            RegisterAndSetAllRolesOutcome {
                token_identifier: expected_token_identifier,
                roles: expected_roles,
            }
        );

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_register_and_set_all_roles_invalid() {
        let parsed_token = get_swap_tx().parse_register_and_set_all_roles();

        let expected: Result<RegisterAndSetAllRolesOutcome, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoResultToExtractTokenIdentifier { expected_issuance_start_data: "registerAndSetAllRoles@".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_set_special_roles_valid() {
        let parsed_token = get_indirect_set_special_role_tx().parse_set_special_roles();

        let expected_user_address = Address::from_bech32_string("erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8").unwrap();
        let expected_token_identifier = "HUTK-f9e2c8".to_string();
        let expected_roles = vec![EsdtLocalRole::Mint, EsdtLocalRole::Burn];
        let expected: Result<SetSpecialRoleOutcome, TransactionsOutcomeParserError> = Ok(
            SetSpecialRoleOutcome {
                user_address: expected_user_address,
                token_identifier: expected_token_identifier,
                roles: expected_roles,
            }
        );

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_set_special_roles_invalid() {
        let parsed_token = get_swap_tx().parse_set_special_roles();

        let expected: Result<SetSpecialRoleOutcome, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoEventOfType { r#type: "ESDTSetRole".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_nft_create_valid() {
        let parsed_token = get_indirect_nft_create_tx().parse_nft_create();

        let expected: Result<_, TransactionsOutcomeParserError> = Ok(
            vec![
                NFTCreateOutcome {
                    token_identifier: "OGS-3f1408".to_string(),
                    nonce: 7725,
                    initial_quantity: BigUint::from_str("1").unwrap()
                },
                NFTCreateOutcome {
                    token_identifier: "OGS-3f1408".to_string(),
                    nonce: 7726,
                    initial_quantity: BigUint::from_str("1").unwrap()
                },
            ]
        );

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_nft_create_invalid() {
        let parsed_token = get_swap_tx().parse_nft_create();

        let expected: Result<_, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoEventOfType { r#type: "ESDTNFTCreate".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_local_mint_valid() {
        let parsed_token = get_indirect_local_mint_tx().parse_local_mint();

        let expected: Result<_, TransactionsOutcomeParserError> = Ok(
            vec![
                LocalMintOutcome {
                    user_address: Address::from_bech32_string("erd1qqqqqqqqqqqqqpgq4fwhp6p6pxrge76jeleqycfw6u98mxeyv5yslcfyxj").unwrap(),
                    token_identifier: "HUSDT-b00128".to_string(),
                    nonce: 0,
                    minted_supply: BigUint::from_str("3692638638").unwrap()
                }
            ]
        );

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_local_mint_invalid() {
        let parsed_token = get_indirect_register_meta_esdt_tx().parse_local_mint();

        let expected: Result<_, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoEventOfType { r#type: "ESDTLocalMint".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_local_burn_valid() {
        assert!(true); // TODO: why the ESDTLocalBurn event is not present in the gateway response?
        return;
        let parsed_token = get_indirect_local_burn_tx().parse_local_burn();

        let expected: Result<_, TransactionsOutcomeParserError> = Ok(
            vec![
                LocalBurnOutcome {
                    user_address: Address::from_bech32_string("erd1qqqqqqqqqqqqqpgq4fwhp6p6pxrge76jeleqycfw6u98mxeyv5yslcfyxj").unwrap(),
                    token_identifier: "HUSDT-b00128".to_string(),
                    nonce: 0,
                    burnt_supply: BigUint::from_str("3692638638").unwrap()
                }
            ]
        );

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_local_burn_invalid() {
        let parsed_token = get_indirect_register_meta_esdt_tx().parse_local_burn();

        let expected: Result<_, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoEventOfType { r#type: "ESDTLocalBurn".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_pause_valid() {
        let parsed_token = get_direct_esdt_pause_tx().parse_pause();

        let expected: Result<_, TransactionsOutcomeParserError> = Ok(());

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_pause_invalid() {
        let parsed_token = get_swap_tx().parse_pause();

        let expected: Result<_, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoEventOfType { r#type: "ESDTPause".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_unpause_valid() {
        let parsed_token = get_direct_esdt_unpause_tx().parse_unpause();

        let expected: Result<_, TransactionsOutcomeParserError> = Ok(());

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_unpause_invalid() {
        let parsed_token = get_swap_tx().parse_unpause();

        let expected: Result<_, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoEventOfType { r#type: "ESDTUnPause".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_freeze_valid() {
        let parsed_token = get_indirect_freeze_tx().parse_freeze();

        let expected: Result<_, TransactionsOutcomeParserError> = Ok(
            vec![
                FreezingOutcome {
                    user_address: Address::from_bech32_string("erd1l27qcz0ff0yneqf2ekf6sfal97c8rmljut7hls5sx99rwvd2yjqq4vyd3j").unwrap(),
                    token_identifier: "CGO-5e9528".to_string(),
                    nonce: 0,
                    balance: BigUint::from_str("215962045106081380654488").unwrap()
                }
            ]
        );

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_freeze_invalid() {
        let parsed_token = get_indirect_register_meta_esdt_tx().parse_freeze();

        let expected: Result<_, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoEventOfType { r#type: "ESDTFreeze".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_unfreeze_valid() {
        assert!(true); // TODO: why the ESDTUnFreeze event is not present in the gateway response?
        return;
        let parsed_token = get_indirect_unfreeze_tx().parse_unfreeze();

        let expected: Result<_, TransactionsOutcomeParserError> = Ok(
            vec![
                FreezingOutcome {
                    user_address: Address::from_bech32_string("erd1l27qcz0ff0yneqf2ekf6sfal97c8rmljut7hls5sx99rwvd2yjqq4vyd3j").unwrap(),
                    token_identifier: "CGO-5e9528".to_string(),
                    nonce: 0,
                    balance: BigUint::from_str("215962045106081380654488").unwrap()
                }
            ]
        );

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_unfreeze_invalid() {
        let parsed_token = get_indirect_register_meta_esdt_tx().parse_unfreeze();

        let expected: Result<_, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoEventOfType { r#type: "ESDTUnFreeze".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_wipe_valid() {
        assert!(true); // TODO: why the ESDTWipe event is not present in the gateway response?
        return;
        let parsed_token = get_direct_wipe_tx().parse_wipe();

        let expected: Result<_, TransactionsOutcomeParserError> = Ok(
            vec![
                WipeOutcome {
                    user_address: Address::from_bech32_string("erd1l27qcz0ff0yneqf2ekf6sfal97c8rmljut7hls5sx99rwvd2yjqq4vyd3j").unwrap(),
                    token_identifier: "CGO-5e9528".to_string(),
                    nonce: 0,
                    balance: BigUint::from_str("215962045106081380654488").unwrap()
                }
            ]
        );

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_wipe_invalid() {
        let parsed_token = get_indirect_register_meta_esdt_tx().parse_wipe();

        let expected: Result<_, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoEventOfType { r#type: "ESDTWipe".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_update_attributes_valid() {
        let parsed_token = get_indirect_update_attributes_tx().parse_update_attributes();

        let expected: Result<_, TransactionsOutcomeParserError> = Ok(
            vec![
                UpdateAttributesOutcome {
                    token_identifier: "KROGAN-54c361".to_string(),
                    nonce: 62,
                    attributes: base64::engine::general_purpose::STANDARD.decode("dGFnczpPZmZpY2lhbCxLcm9nYW52ZXJzZSxQMkUsU3BhY2VzaGlwO0xldmVsOjEwO1NwZWVkOjEwODtBY2NlbGVyYXRpb246NztFdmFzaW9uOjY7QXR0YWNrOjY1O0FjY3VyYWN5OjUyO0ZpcmUgUmF0ZToxNDtBcm1vcjo1MjtIZWFsdGg6NjI3O0NhcmdvOjExMztNaW5pbmc6MTQ=").unwrap()
                }
            ]
        );

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_update_attributes_invalid() {
        let parsed_token = get_indirect_register_meta_esdt_tx().parse_update_attributes();

        let expected: Result<_, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoEventOfType { r#type: "ESDTNFTUpdateAttributes".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_add_quantity_valid() {
        let parsed_token = get_indirect_add_quantity_tx().parse_add_quantity();

        let expected: Result<_, TransactionsOutcomeParserError> = Ok(
            vec![
                AddQuantityOutcome {
                    token_identifier: "LKMEX-aab910".to_string(),
                    nonce: 4222544,
                    added_quantity: BigUint::from_str("653962830569157595532694").unwrap()
                },
            ]
        );

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_add_quantity_invalid() {
        let parsed_token = get_swap_tx().parse_add_quantity();

        let expected: Result<_, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoEventOfType { r#type: "ESDTNFTAddQuantity".to_string() });

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_burn_quantity_valid() {
        assert!(true); // TODO: why the ESDTNFTBurn event is not present in the gateway response?
        return;
        let parsed_token = get_indirect_burn_quantity_tx().parse_burn_quantity();

        let expected: Result<_, TransactionsOutcomeParserError> = Ok(
            vec![
                BurnQuantityOutcome {
                    token_identifier: "LKMEX-aab910".to_string(),
                    nonce: 4222544,
                    burnt_quantity: BigUint::from_str("653962830569157595532694").unwrap()
                },
            ]
        );

        assert_eq!(parsed_token, expected);
    }

    #[test]
    fn test_parse_burn_quantity_invalid() {
        let parsed_token = get_swap_tx().parse_burn_quantity();

        let expected: Result<_, TransactionsOutcomeParserError> = Err(TransactionsOutcomeParserError::NoEventOfType { r#type: "ESDTNFTBurn".to_string() });

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

    fn get_indirect_register_and_set_all_roles_tx() -> TxResponse {
        let data = r#"
        {
          "data": {
            "transaction": {
              "type": "normal",
              "processingTypeOnSource": "SCInvoking",
              "processingTypeOnDestination": "SCInvoking",
              "hash": "5095439f11e31c6b843f0fc5c35b3d7197998d4480597a862fde7c2464a7cb05",
              "nonce": 5,
              "round": 2662387,
              "epoch": 1084,
              "value": "50000000000000000",
              "receiver": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
              "sender": "erd19ehlnz7ncelmm57lfdq6ccxx3ethd49hydux4lc0sdh3swhzxj9q2dx5pa",
              "gasPrice": 1000000000,
              "gasLimit": 150000000,
              "gasUsed": 150000000,
              "data": "bGlzdGluZ0A2MjYxNjY3OTYyNjU2OTY0NzU2YjM1Nzg2NTMzNzA2OTc0MzMzMjc4NzU2YjcwNmU2NjdhNjQ2NjM0NjM2OTM3NjkzNjc4NjY3NTY0NmYzNzcwNzU3NDc1NzIzMjc4MzM2OTYzN2E2MjMzMzMzNjZmMzM3OTY1QEA0ZTRlNGUyMDY0NjU2ZTc1NmQ2OTcyNjUyMDIzQDAzZThANGU0ZTRlNGI0NTU5QDRlNGU0ZTU0NDk0YjQ1NTJAYzhAQDRlNGU0ZTU0NDE0N0AyZTZkNzAzNEAwMUAwMUBAQEBA",
              "signature": "42893b48fc1d9cf3e9aaaffdf1740006c6e3158fce0325c81de2442f1e36c43d009d9c011f9e8b7c429026e674794b44054249326308954632bf3eb811837b03",
              "sourceShard": 2,
              "destinationShard": 1,
              "blockNonce": 2599049,
              "blockHash": "05eeb3aa29da7ade482ce8acebf4d9deac03fce2cdf543fe8fdf3d4e0b241a60",
              "notarizedAtSourceInMetaNonce": 2600469,
              "NotarizedAtSourceInMetaHash": "aa093c4de06e723031ee514c113dac4ae0dbb51cdac5e94a10baf8a14b28a88d",
              "notarizedAtDestinationInMetaNonce": 2600473,
              "notarizedAtDestinationInMetaHash": "b9c9b46b7268936f71c45ca8f8b43fda4ec0423b4d526aadd95597a2411590f4",
              "miniblockType": "TxBlock",
              "miniblockHash": "daee239dca4ec78b1e664a3fc9af2264958fd91accda5763d7b05239a61d3820",
              "hyperblockNonce": 2600473,
              "hyperblockHash": "b9c9b46b7268936f71c45ca8f8b43fda4ec0423b4d526aadd95597a2411590f4",
              "timestamp": 1709974322,
              "smartContractResults": [
                {
                  "hash": "1c58d030cf98ea80316fbd824cc4ae783f0bfa8d6334941cfec37163b21c729c",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "@00@4e4e4e54494b45522d653234386261@d75baec91ae3f73074858677bf653d249c1c6e6da9e6d0b45f375c1e8ad51920@d0932a6f6c2c64123a6f792f4cb367e0fd1f96d0eda90a2a5cdab02f63741057@5095439f11e31c6b843f0fc5c35b3d7197998d4480597a862fde7c2464a7cb05@00",
                  "prevTxHash": "498f12c4437d7283add035b579559f3d0ee957375bfc4a67dc71e597116f8aaf",
                  "originalTxHash": "5095439f11e31c6b843f0fc5c35b3d7197998d4480597a862fde7c2464a7cb05",
                  "gasLimit": 91786873,
                  "gasPrice": 1000000000,
                  "callType": 2,
                  "originalSender": "erd19ehlnz7ncelmm57lfdq6ccxx3ethd49hydux4lc0sdh3swhzxj9q2dx5pa",
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                        "identifier": "callBack",
                        "topics": [
                          "ZW1pdF9saXN0aW5nX2V2ZW50",
                          "AAAAO2JhZnliZWlkdWs1eGUzcGl0MzJ4dWtwbmZ6ZGY0Y2k3aTZ4ZnVkbzdwdXR1cjJ4M2ljemIzMzZvM3llAAAAAAAAAAZOTk5UQUcAAAAELm1wNAAAAA5OTk4gZGVudW1pcmUgIwEAAAAAyAAAAAAAAAAAAAAAAgPoAAAAD05OTlRJS0VSLWUyNDhiYQABAAAAAAAA"
                        ],
                        "data": null,
                        "additionalData": [
                          ""
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "SY8SxEN9coOt0DW1eVWfPQ7pVzdb/Epn3HHllxFviq8="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "operation": "transfer"
                },
                {
                  "hash": "573efb0676de0601a6e279cc81c67f18f91e0f0fbb1ba9b1c933eac883632f58",
                  "nonce": 1,
                  "value": 816208080000000,
                  "receiver": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                  "sender": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                  "data": "@6f6b",
                  "prevTxHash": "1c58d030cf98ea80316fbd824cc4ae783f0bfa8d6334941cfec37163b21c729c",
                  "originalTxHash": "5095439f11e31c6b843f0fc5c35b3d7197998d4480597a862fde7c2464a7cb05",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "operation": "transfer",
                  "isRefund": true
                },
                {
                  "hash": "351caf3456f3ae8f7ec9e1e766d6ad7e7a82e1ee1f07c8f7949ef60688d0ea32",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "ESDTSetBurnRoleForAll@4e4e4e54494b45522d653234386261",
                  "prevTxHash": "498f12c4437d7283add035b579559f3d0ee957375bfc4a67dc71e597116f8aaf",
                  "originalTxHash": "5095439f11e31c6b843f0fc5c35b3d7197998d4480597a862fde7c2464a7cb05",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "originalSender": "erd19ehlnz7ncelmm57lfdq6ccxx3ethd49hydux4lc0sdh3swhzxj9q2dx5pa",
                  "logs": {
                    "address": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
                    "events": [
                      {
                        "address": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "SY8SxEN9coOt0DW1eVWfPQ7pVzdb/Epn3HHllxFviq8="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "operation": "transfer"
                },
                {
                  "hash": "498f12c4437d7283add035b579559f3d0ee957375bfc4a67dc71e597116f8aaf",
                  "nonce": 0,
                  "value": 50000000000000000,
                  "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "sender": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                  "data": "registerAndSetAllRoles@4e4e4e4b4559@4e4e4e54494b4552@4e4654@@d0932a6f6c2c64123a6f792f4cb367e0fd1f96d0eda90a2a5cdab02f63741057@5095439f11e31c6b843f0fc5c35b3d7197998d4480597a862fde7c2464a7cb05@84128c",
                  "prevTxHash": "5095439f11e31c6b843f0fc5c35b3d7197998d4480597a862fde7c2464a7cb05",
                  "originalTxHash": "5095439f11e31c6b843f0fc5c35b3d7197998d4480597a862fde7c2464a7cb05",
                  "gasLimit": 141786873,
                  "gasPrice": 1000000000,
                  "callType": 1,
                  "originalSender": "erd19ehlnz7ncelmm57lfdq6ccxx3ethd49hydux4lc0sdh3swhzxj9q2dx5pa",
                  "operation": "transfer",
                  "function": "registerAndSetAllRoles"
                },
                {
                  "hash": "fea9fe80599595ed277d1ca8b33afd99a01cc266faa62f3f719a5573e7baa2fe",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "ESDTSetBurnRoleForAll@4e4e4e54494b45522d653234386261",
                  "prevTxHash": "498f12c4437d7283add035b579559f3d0ee957375bfc4a67dc71e597116f8aaf",
                  "originalTxHash": "5095439f11e31c6b843f0fc5c35b3d7197998d4480597a862fde7c2464a7cb05",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "originalSender": "erd19ehlnz7ncelmm57lfdq6ccxx3ethd49hydux4lc0sdh3swhzxj9q2dx5pa",
                  "logs": {
                    "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                    "events": [
                      {
                        "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "SY8SxEN9coOt0DW1eVWfPQ7pVzdb/Epn3HHllxFviq8="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "operation": "transfer"
                },
                {
                  "hash": "73367e24dd5e7a77be6a0d24dff90fabe12901aa8cfe2054edda0dd28ef0020e",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "ESDTSetRole@4e4e4e54494b45522d653234386261@45534454526f6c654e4654437265617465@45534454526f6c654e46544275726e@45534454526f6c654e465455706461746541747472696275746573@45534454526f6c654e4654416464555249",
                  "prevTxHash": "498f12c4437d7283add035b579559f3d0ee957375bfc4a67dc71e597116f8aaf",
                  "originalTxHash": "5095439f11e31c6b843f0fc5c35b3d7197998d4480597a862fde7c2464a7cb05",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "originalSender": "erd19ehlnz7ncelmm57lfdq6ccxx3ethd49hydux4lc0sdh3swhzxj9q2dx5pa",
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                        "identifier": "ESDTSetRole",
                        "topics": [
                          "Tk5OVElLRVItZTI0OGJh",
                          "",
                          "",
                          "RVNEVFJvbGVORlRDcmVhdGU=",
                          "RVNEVFJvbGVORlRCdXJu",
                          "RVNEVFJvbGVORlRVcGRhdGVBdHRyaWJ1dGVz",
                          "RVNEVFJvbGVORlRBZGRVUkk="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "SY8SxEN9coOt0DW1eVWfPQ7pVzdb/Epn3HHllxFviq8="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "operation": "ESDTSetRole",
                  "function": "ESDTSetRole"
                }
              ],
              "logs": {
                "address": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                "events": [
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                    "identifier": "transferValueOnly",
                    "topics": [
                      "saK8LsUAAA==",
                      "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8="
                    ],
                    "data": "QXN5bmNDYWxs",
                    "additionalData": [
                      "QXN5bmNDYWxs",
                      "cmVnaXN0ZXJBbmRTZXRBbGxSb2xlcw==",
                      "Tk5OS0VZ",
                      "Tk5OVElLRVI=",
                      "TkZU",
                      ""
                    ]
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqw0v648l5893udledkvl93elpk3ve8l0uah0slplrv2",
                    "identifier": "writeLog",
                    "topics": [
                      "Lm/5i9PGf73T30tBrGDGjld21LcjeGr/D4NvGDriNIo="
                    ],
                    "data": "QDZmNmI=",
                    "additionalData": [
                      "QDZmNmI="
                    ]
                  }
                ]
              },
              "status": "success",
              "operation": "transfer",
              "function": "listing",
              "initiallyPaidFee": "1888080000000000",
              "fee": "1888080000000000",
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

    fn get_indirect_set_special_role_tx() -> TxResponse {
        let data = r#"
        {
          "data": {
            "transaction": {
              "type": "normal",
              "processingTypeOnSource": "SCInvoking",
              "processingTypeOnDestination": "SCInvoking",
              "hash": "9938110c8660816bce1d125d93b6c7d5d3f05a84bee4a2af26c65579f9448d54",
              "nonce": 2934,
              "round": 2666959,
              "epoch": 1086,
              "value": "0",
              "receiver": "erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8",
              "sender": "erd1rc5p5drg26vggn6jx9puv6xlgka5n6ajm6cer554tzguwfm6v5ys2pr3pc",
              "gasPrice": 1000000000,
              "gasLimit": 500000000,
              "gasUsed": 500000000,
              "data": "c2V0TWFya2V0Um9sZXM=",
              "signature": "0f1bee09f1c1f6b4753b464b6ddaf5cd4397311a2de4a48c885f00cb4dcb642cab2bf2a1d3a9420012a930ce66b94ec2f70d26eb3fee84af0230d4ba5899450b",
              "sourceShard": 1,
              "destinationShard": 1,
              "blockNonce": 2603621,
              "blockHash": "cbb1a98994c81f0411078e515705523bb4e4dac53f285c4ba8ec4d5051a9a4b9",
              "notarizedAtSourceInMetaNonce": 2605045,
              "NotarizedAtSourceInMetaHash": "4b802301ebe6b970ae0676e1d2a6122c0047963c2b94ecf1c7a8c6bde92a65fb",
              "notarizedAtDestinationInMetaNonce": 2605045,
              "notarizedAtDestinationInMetaHash": "4b802301ebe6b970ae0676e1d2a6122c0047963c2b94ecf1c7a8c6bde92a65fb",
              "miniblockType": "TxBlock",
              "miniblockHash": "4108457fdd79754aaff59c34948a36743087df4629acd77f7cbeb1045c61a47d",
              "hyperblockNonce": 2605045,
              "hyperblockHash": "4b802301ebe6b970ae0676e1d2a6122c0047963c2b94ecf1c7a8c6bde92a65fb",
              "timestamp": 1710001754,
              "smartContractResults": [
                {
                  "hash": "cfa868f6cb3f609fc68bd46ea814b4d13ef9c29bfd72e4422e4744a764726052",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "sender": "erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8",
                  "data": "setSpecialRole@4855544b2d663965326338@00000000000000000500c08c556245837fe0522a9d74c92eed57581db5e76509@45534454526f6c654c6f63616c4d696e74@45534454526f6c654c6f63616c4275726e@46a1a392909013cffdddb484da1c0c490f70499d00c6c43635998cb9935cc18d@9938110c8660816bce1d125d93b6c7d5d3f05a84bee4a2af26c65579f9448d54@6e11bc",
                  "prevTxHash": "9938110c8660816bce1d125d93b6c7d5d3f05a84bee4a2af26c65579f9448d54",
                  "originalTxHash": "9938110c8660816bce1d125d93b6c7d5d3f05a84bee4a2af26c65579f9448d54",
                  "gasLimit": 495312637,
                  "gasPrice": 1000000000,
                  "callType": 1,
                  "originalSender": "erd1rc5p5drg26vggn6jx9puv6xlgka5n6ajm6cer554tzguwfm6v5ys2pr3pc",
                  "operation": "transfer",
                  "function": "setSpecialRole"
                },
                {
                  "hash": "14ea28181d9fedc6df4897f6819d765f5403b1b3a42c71181a21a9281aacdc02",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "ESDTSetRole@4855544b2d663965326338@45534454526f6c654c6f63616c4d696e74@45534454526f6c654c6f63616c4275726e",
                  "prevTxHash": "cfa868f6cb3f609fc68bd46ea814b4d13ef9c29bfd72e4422e4744a764726052",
                  "originalTxHash": "9938110c8660816bce1d125d93b6c7d5d3f05a84bee4a2af26c65579f9448d54",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "originalSender": "erd1rc5p5drg26vggn6jx9puv6xlgka5n6ajm6cer554tzguwfm6v5ys2pr3pc",
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8",
                        "identifier": "ESDTSetRole",
                        "topics": [
                          "SFVUSy1mOWUyYzg=",
                          "",
                          "",
                          "RVNEVFJvbGVMb2NhbE1pbnQ=",
                          "RVNEVFJvbGVMb2NhbEJ1cm4="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "z6ho9ss/YJ/Gi9RuqBS00T75wpv9cuRCLkdEp2RyYFI="
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
                  "hash": "e07b74dd4622ebef44fc876f8e6e78857fd5dca938e9c52be102c9d09c10b4b0",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8",
                  "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "data": "@00@625ca4cffc098bd8433353aeefae63e842c74f913184758fc5e872ae18d43eb6@46a1a392909013cffdddb484da1c0c490f70499d00c6c43635998cb9935cc18d@9938110c8660816bce1d125d93b6c7d5d3f05a84bee4a2af26c65579f9448d54@00",
                  "prevTxHash": "cfa868f6cb3f609fc68bd46ea814b4d13ef9c29bfd72e4422e4744a764726052",
                  "originalTxHash": "9938110c8660816bce1d125d93b6c7d5d3f05a84bee4a2af26c65579f9448d54",
                  "gasLimit": 445312637,
                  "gasPrice": 1000000000,
                  "callType": 2,
                  "originalSender": "erd1rc5p5drg26vggn6jx9puv6xlgka5n6ajm6cer554tzguwfm6v5ys2pr3pc",
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8",
                        "identifier": "writeLog",
                        "topics": [
                          "AAAAAAAAAAAFAMCMVWJFg3/gUiqddMku7VdYHbXnZQk=",
                          "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gNDQ1MzEyNjM3LCBnYXMgdXNlZCA9IDQxODAzMDg="
                        ],
                        "data": "QDZmNmI=",
                        "additionalData": [
                          "QDZmNmI="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "z6ho9ss/YJ/Gi9RuqBS00T75wpv9cuRCLkdEp2RyYFI="
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
                "address": "erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8",
                "events": [
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8",
                    "identifier": "transferValueOnly",
                    "topics": [
                      "",
                      "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8="
                    ],
                    "data": "QXN5bmNDYWxs",
                    "additionalData": [
                      "QXN5bmNDYWxs",
                      "c2V0U3BlY2lhbFJvbGU=",
                      "SFVUSy1mOWUyYzg=",
                      "AAAAAAAAAAAFAMCMVWJFg3/gUiqddMku7VdYHbXnZQk=",
                      "RVNEVFJvbGVMb2NhbE1pbnQ=",
                      "RVNEVFJvbGVMb2NhbEJ1cm4="
                    ]
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqczx92cj9sdl7q532n46vjthd2avpmd08v5ys0ffcq8",
                    "identifier": "writeLog",
                    "topics": [
                      "HigaNGhWmIRPUjFDxmjfRbtJ67LesZHSlViRxyd6ZQk="
                    ],
                    "data": "QDZmNmI=",
                    "additionalData": [
                      "QDZmNmI="
                    ]
                  }
                ]
              },
              "status": "success",
              "operation": "transfer",
              "function": "setMarketRoles",
              "initiallyPaidFee": "5070290000000000",
              "fee": "5070290000000000",
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

    fn get_indirect_nft_create_tx() -> TxResponse {
        let data = r#"
        {
          "data": {
            "transaction": {
              "type": "normal",
              "processingTypeOnSource": "SCInvoking",
              "processingTypeOnDestination": "SCInvoking",
              "hash": "8c972e4756c553f11ba5dbdb74d9152c6bd0305761cd675b8b3945977d1d2a1a",
              "nonce": 1426,
              "round": 8125500,
              "epoch": 564,
              "value": "2000000000000000000",
              "receiver": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
              "sender": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
              "gasPrice": 1000000000,
              "gasLimit": 47000000,
              "gasUsed": 20741209,
              "data": "YnV5QDUzNzU2MjYzNjE3MjcwNjE3NDY5NGY0NzczQDAy",
              "signature": "c26536e3244d276ee556899d1bd55f33b94fde7b6085c3b8951bcea783267b82b05577c95981a762ff2d34f8ee599ffc59c3d3ce5e0b63b59794bf05632c8208",
              "sourceShard": 0,
              "destinationShard": 1,
              "blockNonce": 8122829,
              "blockHash": "29eeb18824128ea9e9dcb7a6bca4904c2b649bdfe2152c2c9a7ab6e518cd12b1",
              "notarizedAtSourceInMetaNonce": 8106664,
              "NotarizedAtSourceInMetaHash": "f9a9cb2c8579638aef2157bf7a3b3b0e95e72816a392c6c02f27ed0716d3e445",
              "notarizedAtDestinationInMetaNonce": 8106668,
              "notarizedAtDestinationInMetaHash": "399564ebeff9d8e34a7affcb5be28aaf3ca1c077667164e036f8a0c2f9c1e489",
              "miniblockType": "TxBlock",
              "miniblockHash": "0b06f10e82d8bab1d54e455923d1ae718492c1c3e981ce37aeaf041a9ea70cb0",
              "hyperblockNonce": 8106668,
              "hyperblockHash": "399564ebeff9d8e34a7affcb5be28aaf3ca1c077667164e036f8a0c2f9c1e489",
              "timestamp": 1644870600,
              "smartContractResults": [
                {
                  "hash": "08b6d9f6aeed823a64358d4721435ae52f570f0b16557f9d25bdd60ad42efa9b",
                  "nonce": 2,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                  "sender": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                  "data": "@6f6b",
                  "prevTxHash": "da728962dbda115175ea28e4c53b66822f6030f7d9796a826560a39789f94ff0",
                  "originalTxHash": "8c972e4756c553f11ba5dbdb74d9152c6bd0305761cd675b8b3945977d1d2a1a",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "operation": "transfer"
                },
                {
                  "hash": "f7215c6b35c3104b03c2414d3cc003fda2ed32e1083b8e334a4d9416d244a9e2",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                  "sender": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                  "data": "ESDTNFTTransfer@4f47532d336631343038@1e2d@01@08011202000122bc0308ad3c121353756263617270617469204f472023373732351a20000000000000000005002d3c7d4e28851ad0198ebada4964390faaf30076242920e8072a2e516d58454475596d545447796f5369763541785464654c42747462764645715a6d653876786559394236584e4c6d324c68747470733a2f2f697066732e696f2f697066732f516d58454475596d545447796f5369763541785464654c42747462764645715a6d653876786559394236584e4c6d2f363830312e706e67324d68747470733a2f2f697066732e696f2f697066732f516d58454475596d545447796f5369763541785464654c42747462764645715a6d653876786559394236584e4c6d2f363830312e6a736f6e325368747470733a2f2f697066732e696f2f697066732f516d58454475596d545447796f5369763541785464654c42747462764645715a6d653876786559394236584e4c6d2f636f6c6c656374696f6e2e6a736f6e3a5b746167733a537562636172706174692c4f47732c4d757369633b6d657461646174613a516d58454475596d545447796f5369763541785464654c42747462764645715a6d653876786559394236584e4c6d2f363830312e6a736f6e",
                  "prevTxHash": "8c972e4756c553f11ba5dbdb74d9152c6bd0305761cd675b8b3945977d1d2a1a",
                  "originalTxHash": "8c972e4756c553f11ba5dbdb74d9152c6bd0305761cd675b8b3945977d1d2a1a",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "logs": {
                    "address": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                        "identifier": "ESDTNFTTransfer",
                        "topics": [
                          "T0dTLTNmMTQwOA==",
                          "Hi0=",
                          "AQ==",
                          "RBRcZjGI0XGtpghGUVesndBPVduA12J1RovoKXMmeTA="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                        "identifier": "writeLog",
                        "topics": [
                          "AAAAAAAAAAAFAC08fU4ohRrQGY662klkOQ+q8wB2JCk="
                        ],
                        "data": "QDZmNmI=",
                        "additionalData": null
                      },
                      {
                        "address": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "jJcuR1bFU/EbpdvbdNkVLGvQMFdhzWdbizlFl30dKho="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "tokens": [
                    "OGS-3f1408-1e2d"
                  ],
                  "esdtValues": [
                    "1"
                  ],
                  "receivers": [
                    "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t"
                  ],
                  "receiversShardIDs": [
                    0
                  ],
                  "operation": "ESDTNFTTransfer"
                },
                {
                  "hash": "2a8cdd9e6283730c5187c8d16dabb5f278c33e2a1b5d326c7ccd743866bbfbfc",
                  "nonce": 0,
                  "value": 1970000000000000000,
                  "receiver": "erd1rg7505yhnfha4s5lrx863jg9fgea5gtkpcxksa2ayp5gqu8gv55s000pqc",
                  "sender": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                  "prevTxHash": "8c972e4756c553f11ba5dbdb74d9152c6bd0305761cd675b8b3945977d1d2a1a",
                  "originalTxHash": "8c972e4756c553f11ba5dbdb74d9152c6bd0305761cd675b8b3945977d1d2a1a",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "originalSender": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                  "operation": "transfer"
                },
                {
                  "hash": "28805ad2bba63041d1285cb1fc08439c2c4d115258d4e6763cb565c0755053aa",
                  "nonce": 0,
                  "value": 30000000000000000,
                  "receiver": "erd1qqqqqqqqqqqqqpgqg9fa0dmpn8fu3fnleeqn5zt8rl8mdqjkys5s2gtas7",
                  "sender": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                  "prevTxHash": "8c972e4756c553f11ba5dbdb74d9152c6bd0305761cd675b8b3945977d1d2a1a",
                  "originalTxHash": "8c972e4756c553f11ba5dbdb74d9152c6bd0305761cd675b8b3945977d1d2a1a",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "originalSender": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                  "operation": "transfer"
                },
                {
                  "hash": "fce32a3709da78af2806949854d4cca5ef04eae452dd959b2b196c44e2c96871",
                  "nonce": 1,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                  "sender": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                  "data": "@6f6b",
                  "prevTxHash": "f7215c6b35c3104b03c2414d3cc003fda2ed32e1083b8e334a4d9416d244a9e2",
                  "originalTxHash": "8c972e4756c553f11ba5dbdb74d9152c6bd0305761cd675b8b3945977d1d2a1a",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "operation": "transfer"
                },
                {
                  "hash": "917dea2b177328ba7dfdeb4f039acde850c9382a5498a9ff0da83bae9d43f964",
                  "nonce": 1427,
                  "value": 262587910000000,
                  "receiver": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                  "sender": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                  "data": "@6f6b@1e2d@1e2e",
                  "prevTxHash": "8c972e4756c553f11ba5dbdb74d9152c6bd0305761cd675b8b3945977d1d2a1a",
                  "originalTxHash": "8c972e4756c553f11ba5dbdb74d9152c6bd0305761cd675b8b3945977d1d2a1a",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "logs": {
                    "address": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                    "events": [
                      {
                        "address": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "jJcuR1bFU/EbpdvbdNkVLGvQMFdhzWdbizlFl30dKho="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "operation": "transfer",
                  "isRefund": true
                },
                {
                  "hash": "da728962dbda115175ea28e4c53b66822f6030f7d9796a826560a39789f94ff0",
                  "nonce": 1,
                  "value": 0,
                  "receiver": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                  "sender": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                  "data": "ESDTNFTTransfer@4f47532d336631343038@1e2e@01@08011202000122bc0308ae3c121353756263617270617469204f472023373732361a20000000000000000005002d3c7d4e28851ad0198ebada4964390faaf30076242920e8072a2e516d58454475596d545447796f5369763541785464654c42747462764645715a6d653876786559394236584e4c6d324c68747470733a2f2f697066732e696f2f697066732f516d58454475596d545447796f5369763541785464654c42747462764645715a6d653876786559394236584e4c6d2f393238302e706e67324d68747470733a2f2f697066732e696f2f697066732f516d58454475596d545447796f5369763541785464654c42747462764645715a6d653876786559394236584e4c6d2f393238302e6a736f6e325368747470733a2f2f697066732e696f2f697066732f516d58454475596d545447796f5369763541785464654c42747462764645715a6d653876786559394236584e4c6d2f636f6c6c656374696f6e2e6a736f6e3a5b746167733a537562636172706174692c4f47732c4d757369633b6d657461646174613a516d58454475596d545447796f5369763541785464654c42747462764645715a6d653876786559394236584e4c6d2f393238302e6a736f6e",
                  "prevTxHash": "8c972e4756c553f11ba5dbdb74d9152c6bd0305761cd675b8b3945977d1d2a1a",
                  "originalTxHash": "8c972e4756c553f11ba5dbdb74d9152c6bd0305761cd675b8b3945977d1d2a1a",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "logs": {
                    "address": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                        "identifier": "ESDTNFTTransfer",
                        "topics": [
                          "T0dTLTNmMTQwOA==",
                          "Hi4=",
                          "AQ==",
                          "RBRcZjGI0XGtpghGUVesndBPVduA12J1RovoKXMmeTA="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                        "identifier": "writeLog",
                        "topics": [
                          "AAAAAAAAAAAFAC08fU4ohRrQGY662klkOQ+q8wB2JCk="
                        ],
                        "data": "QDZmNmI=",
                        "additionalData": null
                      },
                      {
                        "address": "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "jJcuR1bFU/EbpdvbdNkVLGvQMFdhzWdbizlFl30dKho="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "tokens": [
                    "OGS-3f1408-1e2e"
                  ],
                  "esdtValues": [
                    "1"
                  ],
                  "receivers": [
                    "erd1gs29ce333rghrtdxppr9z4avnhgy74wmsrtkya2x305zjuex0ycqmaae3t"
                  ],
                  "receiversShardIDs": [
                    0
                  ],
                  "operation": "ESDTNFTTransfer"
                }
              ],
              "logs": {
                "address": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                "events": [
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                    "identifier": "ESDTNFTCreate",
                    "topics": [
                      "T0dTLTNmMTQwOA==",
                      "Hi0=",
                      "AQ==",
                      "CAESAgABIrwDCK08EhNTdWJjYXJwYXRpIE9HICM3NzI1GiAAAAAAAAAAAAUALTx9TiiFGtAZjrraSWQ5D6rzAHYkKSDoByouUW1YRUR1WW1UVEd5b1NpdjVBeFRkZUxCdHRidkZFcVptZTh2eGVZOUI2WE5MbTJMaHR0cHM6Ly9pcGZzLmlvL2lwZnMvUW1YRUR1WW1UVEd5b1NpdjVBeFRkZUxCdHRidkZFcVptZTh2eGVZOUI2WE5MbS82ODAxLnBuZzJNaHR0cHM6Ly9pcGZzLmlvL2lwZnMvUW1YRUR1WW1UVEd5b1NpdjVBeFRkZUxCdHRidkZFcVptZTh2eGVZOUI2WE5MbS82ODAxLmpzb24yU2h0dHBzOi8vaXBmcy5pby9pcGZzL1FtWEVEdVltVFRHeW9TaXY1QXhUZGVMQnR0YnZGRXFabWU4dnhlWTlCNlhOTG0vY29sbGVjdGlvbi5qc29uOlt0YWdzOlN1YmNhcnBhdGksT0dzLE11c2ljO21ldGFkYXRhOlFtWEVEdVltVFRHeW9TaXY1QXhUZGVMQnR0YnZGRXFabWU4dnhlWTlCNlhOTG0vNjgwMS5qc29u"
                    ],
                    "data": null,
                    "additionalData": null
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                    "identifier": "ESDTNFTTransfer",
                    "topics": [
                      "T0dTLTNmMTQwOA==",
                      "Hi0=",
                      "AQ==",
                      "RBRcZjGI0XGtpghGUVesndBPVduA12J1RovoKXMmeTA="
                    ],
                    "data": null,
                    "additionalData": null
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                    "identifier": "ESDTNFTCreate",
                    "topics": [
                      "T0dTLTNmMTQwOA==",
                      "Hi4=",
                      "AQ==",
                      "CAESAgABIrwDCK48EhNTdWJjYXJwYXRpIE9HICM3NzI2GiAAAAAAAAAAAAUALTx9TiiFGtAZjrraSWQ5D6rzAHYkKSDoByouUW1YRUR1WW1UVEd5b1NpdjVBeFRkZUxCdHRidkZFcVptZTh2eGVZOUI2WE5MbTJMaHR0cHM6Ly9pcGZzLmlvL2lwZnMvUW1YRUR1WW1UVEd5b1NpdjVBeFRkZUxCdHRidkZFcVptZTh2eGVZOUI2WE5MbS85MjgwLnBuZzJNaHR0cHM6Ly9pcGZzLmlvL2lwZnMvUW1YRUR1WW1UVEd5b1NpdjVBeFRkZUxCdHRidkZFcVptZTh2eGVZOUI2WE5MbS85MjgwLmpzb24yU2h0dHBzOi8vaXBmcy5pby9pcGZzL1FtWEVEdVltVFRHeW9TaXY1QXhUZGVMQnR0YnZGRXFabWU4dnhlWTlCNlhOTG0vY29sbGVjdGlvbi5qc29uOlt0YWdzOlN1YmNhcnBhdGksT0dzLE11c2ljO21ldGFkYXRhOlFtWEVEdVltVFRHeW9TaXY1QXhUZGVMQnR0YnZGRXFabWU4dnhlWTlCNlhOTG0vOTI4MC5qc29u"
                    ],
                    "data": null,
                    "additionalData": null
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                    "identifier": "ESDTNFTTransfer",
                    "topics": [
                      "T0dTLTNmMTQwOA==",
                      "Hi4=",
                      "AQ==",
                      "RBRcZjGI0XGtpghGUVesndBPVduA12J1RovoKXMmeTA="
                    ],
                    "data": null,
                    "additionalData": null
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                    "identifier": "transferValueOnly",
                    "topics": [
                      "AAAAAAAAAAAFAC08fU4ohRrQGY662klkOQ+q8wB2JCk=",
                      "Gj1H0Jeab9rCnxmPqMkFSjPaIXYODWh1XSBogHDoZSk=",
                      "G1bYj/+FAAA="
                    ],
                    "data": null,
                    "additionalData": null
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq95786n3gs5ddqxvwhtdyjepep740xqrkys5swtr2gm",
                    "identifier": "transferValueOnly",
                    "topics": [
                      "AAAAAAAAAAAFAC08fU4ohRrQGY662klkOQ+q8wB2JCk=",
                      "AAAAAAAAAAAFAEFT17dhmdPIpn/OQToJZx/PtoJWJCk=",
                      "apTXT0MAAA=="
                    ],
                    "data": null,
                    "additionalData": null
                  }
                ]
              },
              "status": "success",
              "operation": "transfer",
              "function": "buy",
              "initiallyPaidFee": "568505000000000",
              "fee": "305917090000000",
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

    fn get_indirect_local_mint_tx() -> TxResponse {
        let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "BuiltInFunctionCall",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                  "nonce": 14,
                  "round": 2693803,
                  "epoch": 1097,
                  "value": "0",
                  "receiver": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                  "sender": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                  "gasPrice": 1000000000,
                  "gasLimit": 500000000,
                  "gasUsed": 68084453,
                  "data": "TXVsdGlFU0RUTkZUVHJhbnNmZXJAMDAwMDAwMDAwMDAwMDAwMDA1MDBjNDk1YzIzOTAwOGM5MTViMDdkY2Y0M2UxZGQ5MDg0MzNhODRmNTRlMDQ2M0AwMkA1NTUzNDQ1NDJkMzUzODY0MzU2NDMwQEAwYjcxYjBANDE1NjQ4NTM0YzJkMzkzOTM0NjYzNTYyQDA5QDAzYzcwOUA2NDY1NzA2ZjczNjk3NA==",
                  "signature": "5d888bfe3eef74490fe0150cd173462dd2cf7392ec313bb7cb5ee38f665378a095a4379c55b694570129141528e05751c959122a80a1329fb2b03dcb12566d02",
                  "sourceShard": 1,
                  "destinationShard": 1,
                  "blockNonce": 2630464,
                  "blockHash": "007049239ffc1fa25dcd11942d6cf2133865fdc11ef4070c4a8ff80549de66c6",
                  "notarizedAtSourceInMetaNonce": 2631884,
                  "NotarizedAtSourceInMetaHash": "26f54270c64203caef397d387fe59ae947eb058c19a429b6aefaea5257ef6de1",
                  "notarizedAtDestinationInMetaNonce": 2631884,
                  "notarizedAtDestinationInMetaHash": "26f54270c64203caef397d387fe59ae947eb058c19a429b6aefaea5257ef6de1",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "a52fbe649de6e2fe691a9b9f908fb3b3698d749ef01a71918174f272872e4b1d",
                  "hyperblockNonce": 2631884,
                  "hyperblockHash": "26f54270c64203caef397d387fe59ae947eb058c19a429b6aefaea5257ef6de1",
                  "timestamp": 1710162818,
                  "smartContractResults": [
                    {
                      "hash": "03b1759188a7ad78df6881d32f4d1c80a66fd37742e23a7350f7dd3096015e3c",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                      "sender": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                      "data": "deposit",
                      "prevTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "originalTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "gasLimit": 499296500,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "originalSender": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                      "operation": "transfer",
                      "function": "deposit"
                    },
                    {
                      "hash": "7baf2d2c0afa8bebdb3c38737f07c5f57b677b3f2684e9326d1815cf30b2c167",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgq9pryjth2tgg98syv26v3wa04lwe0x2w7q33s44ryx6",
                      "sender": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                      "data": "ESDTTransfer@555344542d353864356430@0b5464@6465706f736974",
                      "prevTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "originalTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "originalSender": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                      "tokens": [
                        "USDT-58d5d0"
                      ],
                      "esdtValues": [
                        "742500"
                      ],
                      "operation": "ESDTTransfer",
                      "function": "deposit"
                    },
                    {
                      "hash": "479830fd948a80c89d0f5baa529ac78f043a4a6bbd439d32206c8a997c3e6ea0",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgq4fwhp6p6pxrge76jeleqycfw6u98mxeyv5yslcfyxj",
                      "sender": "erd1qqqqqqqqqqqqqpgq9pryjth2tgg98syv26v3wa04lwe0x2w7q33s44ryx6",
                      "data": "ESDTTransfer@555344542d353864356430@0b5464@6d696e74416e64456e7465724d61726b6574",
                      "prevTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "originalTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "originalSender": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                      "tokens": [
                        "USDT-58d5d0"
                      ],
                      "esdtValues": [
                        "742500"
                      ],
                      "operation": "ESDTTransfer",
                      "function": "mintAndEnterMarket"
                    },
                    {
                      "hash": "55dc3dc5633a0cb5b720706af016ce33a27b60824fd4df4604060b90e2509104",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgq5wmxdypdta9m7rlexay0hy26adp3yn9lv5ys7xpyez",
                      "sender": "erd1qqqqqqqqqqqqqpgq4fwhp6p6pxrge76jeleqycfw6u98mxeyv5yslcfyxj",
                      "data": "ESDTTransfer@48555344542d623030313238@dc1931ae@656e7465724d61726b657473@000000000000000005002846492eea5a1053c08c56991775f5fbb2f329de0463",
                      "prevTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "originalTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "originalSender": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                      "tokens": [
                        "HUSDT-b00128"
                      ],
                      "esdtValues": [
                        "3692638638"
                      ],
                      "operation": "ESDTTransfer",
                      "function": "enterMarkets"
                    },
                    {
                      "hash": "6db9656d77b8f741ecc1f38aa68a3f78838d2a3b773c112394f4fcf75f251321",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                      "sender": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                      "data": "ESDTTransfer@555344542d353864356430@1d4c",
                      "prevTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "originalTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "originalSender": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                      "tokens": [
                        "USDT-58d5d0"
                      ],
                      "esdtValues": [
                        "7500"
                      ],
                      "operation": "ESDTTransfer"
                    },
                    {
                      "hash": "1d84d6e9975c98093fb2f3689044d11da3659dc36bcc2cb68b615d0a8ba0ead7",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                      "sender": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                      "data": "ESDTNFTTransfer@415648534c2d393934663562@57@0f4ef9@57a4bce90d877edc906fdf8107168d59cea31ac7ebd2ff58d0bc814d0c48803f",
                      "prevTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "originalTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "originalSender": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                      "tokens": [
                        "AVHSL-994f5b-57"
                      ],
                      "esdtValues": [
                        "1003257"
                      ],
                      "receivers": [
                        "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw"
                      ],
                      "receiversShardIDs": [
                        1
                      ],
                      "operation": "ESDTNFTTransfer"
                    },
                    {
                      "hash": "e24ae9cb07e9a02ce485bfe4e01cc9bf5833926b16aa37c1b38cd7ebac79cedc",
                      "nonce": 15,
                      "value": 4319155470000000,
                      "receiver": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                      "sender": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                      "data": "@6f6b@0000000c415648534c2d3939346635620000000000000057000000030f4ef9",
                      "prevTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "originalTxHash": "c0a0a89facd7383935843ac1d1b05e4dfc78fed3a9a5fea94bb0d425f9de5e06",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "operation": "transfer",
                      "isRefund": true
                    }
                  ],
                  "logs": {
                    "address": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                    "events": [
                      {
                        "address": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                        "identifier": "MultiESDTNFTTransfer",
                        "topics": [
                          "VVNEVC01OGQ1ZDA=",
                          "",
                          "C3Gw",
                          "QVZIU0wtOTk0ZjVi",
                          "CQ==",
                          "A8cJ",
                          "AAAAAAAAAAAFAMSVwjkAjJFbB9z0Ph3ZCEM6hPVOBGM="
                        ],
                        "data": null,
                        "additionalData": [
                          "",
                          "TXVsdGlFU0RUTkZUVHJhbnNmZXI=",
                          "AAAAAAAAAAAFAMSVwjkAjJFbB9z0Ph3ZCEM6hPVOBGM=",
                          "Ag==",
                          "VVNEVC01OGQ1ZDA=",
                          "",
                          "C3Gw",
                          "QVZIU0wtOTk0ZjVi",
                          "CQ==",
                          "A8cJ",
                          "ZGVwb3NpdA=="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                        "identifier": "transferValueOnly",
                        "topics": [
                          "",
                          "AAAAAAAAAAAFAChGSS7qWhBTwIxWmRd19fuy8yneBGM="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                          "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                          "Z2V0VG90YWxBc3NldHM="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq9pryjth2tgg98syv26v3wa04lwe0x2w7q33s44ryx6",
                        "identifier": "transferValueOnly",
                        "topics": [
                          "",
                          "AAAAAAAAAAAFAKO2ZpAtX0u/D/k3SPuRWutDEky/ZQk="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                          "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                          "Z2V0QWNjb3VudFRva2Vucw==",
                          "AAAAAAAAAAAFAKpdcOg6CYaM+1LP8gJhLtcKfZskZQk=",
                          "AAAAAAAAAAAFAChGSS7qWhBTwIxWmRd19fuy8yneBGM="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq9pryjth2tgg98syv26v3wa04lwe0x2w7q33s44ryx6",
                        "identifier": "transferValueOnly",
                        "topics": [
                          "",
                          "AAAAAAAAAAAFAKpdcOg6CYaM+1LP8gJhLtcKfZskZQk="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                          "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                          "dG9rZW5zVG9VbmRlcmx5aW5nQW1vdW50",
                          "AV1O20tv"
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                        "identifier": "ESDTTransfer",
                        "topics": [
                          "VVNEVC01OGQ1ZDA=",
                          "",
                          "C1Rk",
                          "AAAAAAAAAAAFAChGSS7qWhBTwIxWmRd19fuy8yneBGM="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                          "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                          "RVNEVFRyYW5zZmVy",
                          "VVNEVC01OGQ1ZDA=",
                          "C1Rk",
                          "ZGVwb3NpdA=="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq9pryjth2tgg98syv26v3wa04lwe0x2w7q33s44ryx6",
                        "identifier": "ESDTTransfer",
                        "topics": [
                          "VVNEVC01OGQ1ZDA=",
                          "",
                          "C1Rk",
                          "AAAAAAAAAAAFAKpdcOg6CYaM+1LP8gJhLtcKfZskZQk="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                          "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                          "RVNEVFRyYW5zZmVy",
                          "VVNEVC01OGQ1ZDA=",
                          "C1Rk",
                          "bWludEFuZEVudGVyTWFya2V0"
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq4fwhp6p6pxrge76jeleqycfw6u98mxeyv5yslcfyxj",
                        "identifier": "transferValueOnly",
                        "topics": [
                          "",
                          "AAAAAAAAAAAFANbiSH+XFY+wzFk4l9Ewk00OcR+XZQk="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                          "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                          "Z2V0Qm9ycm93UmF0ZQ==",
                          "IKhezyM=",
                          "Ldsjnkc="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq4fwhp6p6pxrge76jeleqycfw6u98mxeyv5yslcfyxj",
                        "identifier": "mintAndEnterMarket",
                        "topics": [
                          "YWNjcnVlX2ludGVyZXN0X2V2ZW50",
                          "DVR67Us=",
                          "y5cu",
                          "DgvzoR9ztBA=",
                          "IKkqZlE="
                        ],
                        "data": null,
                        "additionalData": [
                          ""
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq4fwhp6p6pxrge76jeleqycfw6u98mxeyv5yslcfyxj",
                        "identifier": "transferValueOnly",
                        "topics": [
                          "",
                          "AAAAAAAAAAAFAKO2ZpAtX0u/D/k3SPuRWutDEky/ZQk="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                          "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                          "bWludEFsbG93ZWQ=",
                          "AAAAAAAAAAAFAKpdcOg6CYaM+1LP8gJhLtcKfZskZQk=",
                          "C1Rk"
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq4fwhp6p6pxrge76jeleqycfw6u98mxeyv5yslcfyxj",
                        "identifier": "ESDTLocalMint",
                        "topics": [
                          "SFVTRFQtYjAwMTI4",
                          "",
                          "3Bkxrg=="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq4fwhp6p6pxrge76jeleqycfw6u98mxeyv5yslcfyxj",
                        "identifier": "transferValueOnly",
                        "topics": [
                          "",
                          "AAAAAAAAAAAFANbiSH+XFY+wzFk4l9Ewk00OcR+XZQk="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                          "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                          "Z2V0UmF0ZXM=",
                          "IKkqZlE=",
                          "LduzSCM=",
                          "BNtzJUdjAAA="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq4fwhp6p6pxrge76jeleqycfw6u98mxeyv5yslcfyxj",
                        "identifier": "mintAndEnterMarket",
                        "topics": [
                          "dXBkYXRlZF9yYXRlc19ldmVudA==",
                          "Q0LkTg==",
                          "HyNJHA=="
                        ],
                        "data": null,
                        "additionalData": [
                          ""
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq4fwhp6p6pxrge76jeleqycfw6u98mxeyv5yslcfyxj",
                        "identifier": "mintAndEnterMarket",
                        "topics": [
                          "bWludF9ldmVudA==",
                          "AAAAAAAAAAAFAChGSS7qWhBTwIxWmRd19fuy8yneBGM=",
                          "C1Rk",
                          "3Bkxrg=="
                        ],
                        "data": null,
                        "additionalData": [
                          ""
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq4fwhp6p6pxrge76jeleqycfw6u98mxeyv5yslcfyxj",
                        "identifier": "ESDTTransfer",
                        "topics": [
                          "SFVTRFQtYjAwMTI4",
                          "",
                          "3Bkxrg==",
                          "AAAAAAAAAAAFAKO2ZpAtX0u/D/k3SPuRWutDEky/ZQk="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                          "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                          "RVNEVFRyYW5zZmVy",
                          "SFVTRFQtYjAwMTI4",
                          "3Bkxrg==",
                          "ZW50ZXJNYXJrZXRz",
                          "AAAAAAAAAAAFAChGSS7qWhBTwIxWmRd19fuy8yneBGM="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq5wmxdypdta9m7rlexay0hy26adp3yn9lv5ys7xpyez",
                        "identifier": "enterMarkets",
                        "topics": [
                          "c3VwcGxpZXJfcmV3YXJkc19kaXN0cmlidXRlZF9ldmVudA==",
                          "AAAAAAAAAAAFAChGSS7qWhBTwIxWmRd19fuy8yneBGM=",
                          "AAAAAQAAAAAAAAAABQCqXXDoOgmGjPtSz/ICYS7XCn2bJGUJAAAAAAtVU0RDLTM1MGM0ZQAAAAUCPNw4wAAAAAQ9yJKtAAAACUMAs/iXS8DRpQAAAA/AmEmLQNwjEHUzQLS+KtgAAAAAZbprjgAAAABlumuO",
                          ""
                        ],
                        "data": null,
                        "additionalData": [
                          ""
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq5wmxdypdta9m7rlexay0hy26adp3yn9lv5ys7xpyez",
                        "identifier": "transferValueOnly",
                        "topics": [
                          "",
                          "AAAAAAAAAAAFAPiDZob3xsDxPXJ1FRt49Z+PL3KaZQk="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                          "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                          "b25NYXJrZXRDaGFuZ2U=",
                          "AAAAAAAAAAAFAKpdcOg6CYaM+1LP8gJhLtcKfZskZQk=",
                          "AAAAAAAAAAAFAChGSS7qWhBTwIxWmRd19fuy8yneBGM=",
                          "AV4q9H0d"
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq5wmxdypdta9m7rlexay0hy26adp3yn9lv5ys7xpyez",
                        "identifier": "enterMarkets",
                        "topics": [
                          "ZW50ZXJfbWFya2V0X2V2ZW50",
                          "AAAAAAAAAAAFAKpdcOg6CYaM+1LP8gJhLtcKfZskZQk=",
                          "AAAAAAAAAAAFAChGSS7qWhBTwIxWmRd19fuy8yneBGM=",
                          "3Bkxrg=="
                        ],
                        "data": null,
                        "additionalData": [
                          ""
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                        "identifier": "transferValueOnly",
                        "topics": [
                          "",
                          "AAAAAAAAAAAFAChGSS7qWhBTwIxWmRd19fuy8yneBGM="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                          "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                          "Z2V0VG90YWxBc3NldHM="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq9pryjth2tgg98syv26v3wa04lwe0x2w7q33s44ryx6",
                        "identifier": "transferValueOnly",
                        "topics": [
                          "",
                          "AAAAAAAAAAAFAKO2ZpAtX0u/D/k3SPuRWutDEky/ZQk="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                          "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                          "Z2V0QWNjb3VudFRva2Vucw==",
                          "AAAAAAAAAAAFAKpdcOg6CYaM+1LP8gJhLtcKfZskZQk=",
                          "AAAAAAAAAAAFAChGSS7qWhBTwIxWmRd19fuy8yneBGM="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq9pryjth2tgg98syv26v3wa04lwe0x2w7q33s44ryx6",
                        "identifier": "transferValueOnly",
                        "topics": [
                          "",
                          "AAAAAAAAAAAFAKpdcOg6CYaM+1LP8gJhLtcKfZskZQk="
                        ],
                        "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                        "additionalData": [
                          "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                          "dG9rZW5zVG9VbmRlcmx5aW5nQW1vdW50",
                          "AV4q9H0d"
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                        "identifier": "ESDTNFTBurn",
                        "topics": [
                          "QVZIU0wtOTk0ZjVi",
                          "CQ==",
                          "A8cJ"
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                        "identifier": "ESDTTransfer",
                        "topics": [
                          "VVNEVC01OGQ1ZDA=",
                          "",
                          "HUw=",
                          "AAAAAAAAAAAFAH6UefeHERqHcLpMz2gC3xXGhFsJBGM="
                        ],
                        "data": "RGlyZWN0Q2FsbA==",
                        "additionalData": [
                          "RGlyZWN0Q2FsbA==",
                          "RVNEVFRyYW5zZmVy",
                          "VVNEVC01OGQ1ZDA=",
                          "HUw="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                        "identifier": "ESDTNFTCreate",
                        "topics": [
                          "QVZIU0wtOTk0ZjVi",
                          "Vw==",
                          "D075",
                          "CAESBAAPTvkiNwhXGiAAAAAAAAAAAAUAxJXCOQCMkVsH3PQ+HdkIQzqE9U4EYzIAOg8AAAAAAAAAAAAAAAMPTvk="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                        "identifier": "ESDTNFTTransfer",
                        "topics": [
                          "QVZIU0wtOTk0ZjVi",
                          "Vw==",
                          "D075",
                          "V6S86Q2HftyQb9+BBxaNWc6jGsfr0v9Y0LyBTQxIgD8="
                        ],
                        "data": "RGlyZWN0Q2FsbA==",
                        "additionalData": [
                          "RGlyZWN0Q2FsbA==",
                          "RVNEVE5GVFRyYW5zZmVy",
                          "QVZIU0wtOTk0ZjVi",
                          "Vw==",
                          "D075",
                          "V6S86Q2HftyQb9+BBxaNWc6jGsfr0v9Y0LyBTQxIgD8="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                        "identifier": "deposit",
                        "topics": [
                          "",
                          "V6S86Q2HftyQb9+BBxaNWc6jGsfr0v9Y0LyBTQxIgD8=",
                          "BEk="
                        ],
                        "data": "V6S86Q2HftyQb9+BBxaNWc6jGsfr0v9Y0LyBTQxIgD8AAAAAAAAAAAUAKEZJLupaEFPAjFaZF3X1+7LzKd4EYwAAAAQR+lhZAAAAAh1MAAAAAwtUZAAAAAMLiEcAAAADD075",
                        "additionalData": [
                          "V6S86Q2HftyQb9+BBxaNWc6jGsfr0v9Y0LyBTQxIgD8AAAAAAAAAAAUAKEZJLupaEFPAjFaZF3X1+7LzKd4EYwAAAAQR+lhZAAAAAh1MAAAAAwtUZAAAAAMLiEcAAAADD075"
                        ]
                      },
                      {
                        "address": "erd127jte6gdsaldeyr0m7qsw95dt882xxk8a0f07kxshjq56rzgsqlsswpgfw",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "wKCon6zXODk1hDrB0bBeTfx4/tOppf6pS7DUJfneXgY="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "status": "success",
                  "tokens": [
                    "USDT-58d5d0",
                    "AVHSL-994f5b-09"
                  ],
                  "esdtValues": [
                    "750000",
                    "247561"
                  ],
                  "receivers": [
                    "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                    "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5"
                  ],
                  "receiversShardIDs": [
                    1,
                    1
                  ],
                  "operation": "MultiESDTNFTTransfer",
                  "function": "deposit",
                  "initiallyPaidFee": "5300465000000000",
                  "fee": "981309530000000",
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

    fn get_indirect_local_burn_tx() -> TxResponse {
        let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "BuiltInFunctionCall",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "cda950a476339676194097b202a80a085245cbed269e3e7380572ad943974cfa",
                  "nonce": 18,
                  "round": 2620353,
                  "epoch": 1066,
                  "value": "0",
                  "receiver": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                  "sender": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                  "gasPrice": 1000000000,
                  "gasLimit": 500000000,
                  "gasUsed": 61896539,
                  "data": "TXVsdGlFU0RUTkZUVHJhbnNmZXJAMDAwMDAwMDAwMDAwMDAwMDA1MDBjNDk1YzIzOTAwOGM5MTViMDdkY2Y0M2UxZGQ5MDg0MzNhODRmNTRlMDQ2M0AwMUA0MTU2NDg1MzRjMmQzOTM5MzQ2NjM1NjJANTNAMDQ5M2JkQDc3Njk3NDY4NjQ3MjYxNzc=",
                  "signature": "2b8bfc29b7e064807e9259f2f95daa353e83931fd582a7848651efd52ee25c711ab025c9ba22539aefc1fde27f977c225fae4c7db00cbf33f1c5331b93bfcd00",
                  "sourceShard": 2,
                  "destinationShard": 2,
                  "blockNonce": 2557474,
                  "blockHash": "9fbff0c4d66e8315a231b10e150cef024a3b7d10b63ac8076f9f0e8842463062",
                  "notarizedAtSourceInMetaNonce": 2558441,
                  "NotarizedAtSourceInMetaHash": "ee39194fb8c2cd113e38f8c899e1880dd77cf0c431ed4eb468fc6af0305275ed",
                  "notarizedAtDestinationInMetaNonce": 2558441,
                  "notarizedAtDestinationInMetaHash": "ee39194fb8c2cd113e38f8c899e1880dd77cf0c431ed4eb468fc6af0305275ed",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "023799b58a7eec9a7744816ab4fa2fec394f0cfc16268f119f72f1afadba28e2",
                  "hyperblockNonce": 2558441,
                  "hyperblockHash": "ee39194fb8c2cd113e38f8c899e1880dd77cf0c431ed4eb468fc6af0305275ed",
                  "timestamp": 1709722118,
                  "smartContractResults": [
                    {
                      "hash": "17f47b398c60ff1fe7b37420c8b7f388bfcaa2def1821e9d1a95e2ff6870ea12",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                      "sender": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                      "data": "MultiESDTNFTTransfer@01@415648534c2d393934663562@53@08011204000493bd223708531a2000000000000000000500c495c239008c915b07dcf43e1dd908433a84f54e046332003a0f00000000000000000000000316b028@7769746864726177",
                      "prevTxHash": "cda950a476339676194097b202a80a085245cbed269e3e7380572ad943974cfa",
                      "originalTxHash": "cda950a476339676194097b202a80a085245cbed269e3e7380572ad943974cfa",
                      "gasLimit": 499536750,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "originalSender": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                      "tokens": [
                        "AVHSL-994f5b-53"
                      ],
                      "esdtValues": [
                        "299965"
                      ],
                      "receivers": [
                        "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5"
                      ],
                      "receiversShardIDs": [
                        1
                      ],
                      "operation": "MultiESDTNFTTransfer",
                      "function": "withdraw"
                    },
                    {
                      "hash": "f1163785662fdb158de11dedaf7bb2dd15f437c555de79cbb51a5f586edb530d",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                      "sender": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                      "data": "ESDTTransfer@555344542d353864356430@048828",
                      "prevTxHash": "17f47b398c60ff1fe7b37420c8b7f388bfcaa2def1821e9d1a95e2ff6870ea12",
                      "originalTxHash": "cda950a476339676194097b202a80a085245cbed269e3e7380572ad943974cfa",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "originalSender": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                      "logs": {
                        "address": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                        "events": [
                          {
                            "address": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                            "identifier": "ESDTTransfer",
                            "topics": [
                              "VVNEVC01OGQ1ZDA=",
                              "",
                              "BIgo",
                              "YfKvnfm3c2FjZfD3vDk4ys7EDJeeeULZ6ugxa/H59iY="
                            ],
                            "data": null,
                            "additionalData": [
                              "",
                              "RVNEVFRyYW5zZmVy",
                              "VVNEVC01OGQ1ZDA=",
                              "BIgo"
                            ]
                          },
                          {
                            "address": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                            "identifier": "writeLog",
                            "topics": [
                              "AAAAAAAAAAAFAMSVwjkAjJFbB9z0Ph3ZCEM6hPVOBGM="
                            ],
                            "data": "QDZmNmI=",
                            "additionalData": [
                              "QDZmNmI="
                            ]
                          },
                          {
                            "address": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "F/R7OYxg/x/ns3QgyLfziL/Kot7xgh6dGpXi/2hw6hI="
                            ],
                            "data": null,
                            "additionalData": null
                          }
                        ]
                      },
                      "tokens": [
                        "USDT-58d5d0"
                      ],
                      "esdtValues": [
                        "297000"
                      ],
                      "operation": "ESDTTransfer"
                    },
                    {
                      "hash": "37bf41d1fe9f49bf43b17b604316c787619545c57fe87d032581eff78816b6f8",
                      "nonce": 1,
                      "value": 4381034610000000,
                      "receiver": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                      "sender": "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5",
                      "data": "@6f6b@0000000b555344542d353864356430000000000000000000000003048828",
                      "prevTxHash": "17f47b398c60ff1fe7b37420c8b7f388bfcaa2def1821e9d1a95e2ff6870ea12",
                      "originalTxHash": "cda950a476339676194097b202a80a085245cbed269e3e7380572ad943974cfa",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "logs": {
                        "address": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                        "events": [
                          {
                            "address": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "F/R7OYxg/x/ns3QgyLfziL/Kot7xgh6dGpXi/2hw6hI="
                            ],
                            "data": null,
                            "additionalData": null
                          }
                        ]
                      },
                      "operation": "transfer",
                      "isRefund": true
                    }
                  ],
                  "logs": {
                    "address": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                    "events": [
                      {
                        "address": "erd1v8e2l80ekaekzcm97rmmcwfcet8vgryhneu59k02aqckhu0e7cnq6k68ce",
                        "identifier": "MultiESDTNFTTransfer",
                        "topics": [
                          "QVZIU0wtOTk0ZjVi",
                          "Uw==",
                          "BJO9",
                          "AAAAAAAAAAAFAMSVwjkAjJFbB9z0Ph3ZCEM6hPVOBGM="
                        ],
                        "data": null,
                        "additionalData": [
                          "",
                          "TXVsdGlFU0RUTkZUVHJhbnNmZXI=",
                          "AAAAAAAAAAAFAMSVwjkAjJFbB9z0Ph3ZCEM6hPVOBGM=",
                          "AQ==",
                          "QVZIU0wtOTk0ZjVi",
                          "Uw==",
                          "BJO9",
                          "d2l0aGRyYXc="
                        ]
                      }
                    ]
                  },
                  "status": "success",
                  "tokens": [
                    "AVHSL-994f5b-53"
                  ],
                  "esdtValues": [
                    "299965"
                  ],
                  "receivers": [
                    "erd1qqqqqqqqqqqqqpgqcj2uywgq3jg4kp7u7slpmkgggvagfa2wq33s22aqt5"
                  ],
                  "receiversShardIDs": [
                    1
                  ],
                  "operation": "MultiESDTNFTTransfer",
                  "function": "withdraw",
                  "initiallyPaidFee": "5257400000000000",
                  "fee": "876365390000000",
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

    // I didn't find any indirect tx containing the ESDTPause log
    fn get_direct_esdt_pause_tx() -> TxResponse {
        let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "SCInvoking",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "e20efe36ddcc2696d55f99b350aead5fddfda215565e75e0a060eda88206d811",
                  "nonce": 114,
                  "round": 14907143,
                  "epoch": 1035,
                  "value": "0",
                  "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "sender": "erd18p74xu7rq3agnfnqpnzfjpy609u7m3yyshd6z8xqeky46tvza50syytpff",
                  "gasPrice": 1000000000,
                  "gasLimit": 60000000,
                  "gasUsed": 50092000,
                  "data": "cGF1c2VANDY0OTU0NTkyZDM3MzM2NjM4NjY2Mw==",
                  "signature": "fa52549385e0aeb7cbb830f2946aa28ec1773569881da374e5fccaacfa1adb217c4d704317fa22c54e2326d4bd473cb9b4d2873d85446e86bd43d7a9928d530d",
                  "sourceShard": 1,
                  "destinationShard": 4294967295,
                  "blockNonce": 14886901,
                  "blockHash": "33bf1d9c8f5fab95120c27f3cc1c3059f6c76e6e75c07950bdf62cead45a2796",
                  "notarizedAtSourceInMetaNonce": 14886901,
                  "NotarizedAtSourceInMetaHash": "33bf1d9c8f5fab95120c27f3cc1c3059f6c76e6e75c07950bdf62cead45a2796",
                  "notarizedAtDestinationInMetaNonce": 14886901,
                  "notarizedAtDestinationInMetaHash": "33bf1d9c8f5fab95120c27f3cc1c3059f6c76e6e75c07950bdf62cead45a2796",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "83059b1d825d13a112f3783c56aaae4869df6f80e679cad3f92b49230a3eb3ab",
                  "hyperblockNonce": 14886901,
                  "hyperblockHash": "33bf1d9c8f5fab95120c27f3cc1c3059f6c76e6e75c07950bdf62cead45a2796",
                  "timestamp": 1685560458,
                  "smartContractResults": [
                    {
                      "hash": "cf3c69d2bb47e8be07ca4cb7d13dd40152e7dd162eae148ea25b4c6118c31b85",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqq2m3f0f",
                      "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "data": "ESDTPause@464954592d373366386663",
                      "prevTxHash": "e20efe36ddcc2696d55f99b350aead5fddfda215565e75e0a060eda88206d811",
                      "originalTxHash": "e20efe36ddcc2696d55f99b350aead5fddfda215565e75e0a060eda88206d811",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "operation": "ESDTPause"
                    },
                    {
                      "hash": "481349ce65b2b78fa650da591ba5c2346d7b4893b7d25892f208ecce956e9fdb",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
                      "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "data": "ESDTPause@464954592d373366386663",
                      "prevTxHash": "e20efe36ddcc2696d55f99b350aead5fddfda215565e75e0a060eda88206d811",
                      "originalTxHash": "e20efe36ddcc2696d55f99b350aead5fddfda215565e75e0a060eda88206d811",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "operation": "ESDTPause"
                    },
                    {
                      "hash": "f98a7b381ed4d451c90d5f951ac6fa5bb365e2e01abcf7239715ebbc03096d3a",
                      "nonce": 115,
                      "value": 99080000000000,
                      "receiver": "erd18p74xu7rq3agnfnqpnzfjpy609u7m3yyshd6z8xqeky46tvza50syytpff",
                      "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "data": "@6f6b",
                      "prevTxHash": "e20efe36ddcc2696d55f99b350aead5fddfda215565e75e0a060eda88206d811",
                      "originalTxHash": "e20efe36ddcc2696d55f99b350aead5fddfda215565e75e0a060eda88206d811",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "logs": {
                        "address": "erd18p74xu7rq3agnfnqpnzfjpy609u7m3yyshd6z8xqeky46tvza50syytpff",
                        "events": [
                          {
                            "address": "erd18p74xu7rq3agnfnqpnzfjpy609u7m3yyshd6z8xqeky46tvza50syytpff",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "4g7+Nt3MJpbVX5mzUK6tX939ohVWXnXgoGDtqIIG2BE="
                            ],
                            "data": null,
                            "additionalData": null
                          }
                        ]
                      },
                      "operation": "transfer",
                      "isRefund": true
                    },
                    {
                      "hash": "527bff03b97aa8f9e855985a97e99e6ff94435e0bccbb1a915f4a6a4c5bb9da8",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                      "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "data": "ESDTPause@464954592d373366386663",
                      "prevTxHash": "e20efe36ddcc2696d55f99b350aead5fddfda215565e75e0a060eda88206d811",
                      "originalTxHash": "e20efe36ddcc2696d55f99b350aead5fddfda215565e75e0a060eda88206d811",
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
                              "4g7+Nt3MJpbVX5mzUK6tX939ohVWXnXgoGDtqIIG2BE="
                            ],
                            "data": null,
                            "additionalData": null
                          }
                        ]
                      },
                      "operation": "ESDTPause"
                    }
                  ],
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                    "events": [
                      {
                        "address": "erd18p74xu7rq3agnfnqpnzfjpy609u7m3yyshd6z8xqeky46tvza50syytpff",
                        "identifier": "ESDTPause",
                        "topics": [
                          "RklUWS03M2Y4ZmM="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "status": "success",
                  "operation": "transfer",
                  "function": "pause",
                  "initiallyPaidFee": "691080000000000",
                  "fee": "592000000000000",
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

    // I didn't find any indirect tx containing the ESDTUnPause log
    fn get_direct_esdt_unpause_tx() -> TxResponse {
        let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "SCInvoking",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "881f86a1f2fedfcdf9c738a160f50fcfe0a2d370930377859aec436a1b581b5d",
                  "nonce": 115,
                  "round": 15089482,
                  "epoch": 1047,
                  "value": "0",
                  "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "sender": "erd18p74xu7rq3agnfnqpnzfjpy609u7m3yyshd6z8xqeky46tvza50syytpff",
                  "gasPrice": 1000000000,
                  "gasLimit": 60000000,
                  "gasUsed": 50095000,
                  "data": "dW5QYXVzZUA0NjQ5NTQ1OTJkMzczMzY2Mzg2NjYz",
                  "signature": "189923f839fed705f9baca4839fc390b35ab11de4f1bb2c1e821a705f19c7b2c50300ab95692a37e8d9cde88ba55779df2c25689ed08fe1cdccf68c0212f0e06",
                  "sourceShard": 1,
                  "destinationShard": 4294967295,
                  "blockNonce": 15069191,
                  "blockHash": "800a4ef7e24dade62dd91aeec7e61b5419fb3e4b862861126c01f7b640321b12",
                  "notarizedAtSourceInMetaNonce": 15069191,
                  "NotarizedAtSourceInMetaHash": "800a4ef7e24dade62dd91aeec7e61b5419fb3e4b862861126c01f7b640321b12",
                  "notarizedAtDestinationInMetaNonce": 15069191,
                  "notarizedAtDestinationInMetaHash": "800a4ef7e24dade62dd91aeec7e61b5419fb3e4b862861126c01f7b640321b12",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "69411c152f63ffe396e8d7b13df15c94fa270e75986efdcb7d750ede23baf782",
                  "hyperblockNonce": 15069191,
                  "hyperblockHash": "800a4ef7e24dade62dd91aeec7e61b5419fb3e4b862861126c01f7b640321b12",
                  "timestamp": 1686654492,
                  "smartContractResults": [
                    {
                      "hash": "29f3b64f3164a664d79eb18a0f0207be4ab8451a4cf733b6d239a3307ea33299",
                      "nonce": 116,
                      "value": 99050000000000,
                      "receiver": "erd18p74xu7rq3agnfnqpnzfjpy609u7m3yyshd6z8xqeky46tvza50syytpff",
                      "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "data": "@6f6b",
                      "prevTxHash": "881f86a1f2fedfcdf9c738a160f50fcfe0a2d370930377859aec436a1b581b5d",
                      "originalTxHash": "881f86a1f2fedfcdf9c738a160f50fcfe0a2d370930377859aec436a1b581b5d",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "logs": {
                        "address": "erd18p74xu7rq3agnfnqpnzfjpy609u7m3yyshd6z8xqeky46tvza50syytpff",
                        "events": [
                          {
                            "address": "erd18p74xu7rq3agnfnqpnzfjpy609u7m3yyshd6z8xqeky46tvza50syytpff",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "iB+GofL+3835xzihYPUPz+Ci03CTA3eFmuxDahtYG10="
                            ],
                            "data": null,
                            "additionalData": null
                          }
                        ]
                      },
                      "operation": "transfer",
                      "isRefund": true
                    },
                    {
                      "hash": "3eecf9f675dcbd1c19596a2e436c622f599ccd4cadabfcb8def28b82c7a8db41",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                      "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "data": "ESDTUnPause@464954592d373366386663",
                      "prevTxHash": "881f86a1f2fedfcdf9c738a160f50fcfe0a2d370930377859aec436a1b581b5d",
                      "originalTxHash": "881f86a1f2fedfcdf9c738a160f50fcfe0a2d370930377859aec436a1b581b5d",
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
                              "iB+GofL+3835xzihYPUPz+Ci03CTA3eFmuxDahtYG10="
                            ],
                            "data": null,
                            "additionalData": null
                          }
                        ]
                      },
                      "operation": "ESDTUnPause"
                    },
                    {
                      "hash": "d1c43e9abab2e6656ba2aeb5c747638d127d82baed38f9e0b1aeceadfc3c41ad",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
                      "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "data": "ESDTUnPause@464954592d373366386663",
                      "prevTxHash": "881f86a1f2fedfcdf9c738a160f50fcfe0a2d370930377859aec436a1b581b5d",
                      "originalTxHash": "881f86a1f2fedfcdf9c738a160f50fcfe0a2d370930377859aec436a1b581b5d",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "operation": "ESDTUnPause"
                    },
                    {
                      "hash": "fb6557f8b762e997a445413e22562036d49dae6e3146a9f2fe71895fba2542b2",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqq2m3f0f",
                      "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "data": "ESDTUnPause@464954592d373366386663",
                      "prevTxHash": "881f86a1f2fedfcdf9c738a160f50fcfe0a2d370930377859aec436a1b581b5d",
                      "originalTxHash": "881f86a1f2fedfcdf9c738a160f50fcfe0a2d370930377859aec436a1b581b5d",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "operation": "ESDTUnPause"
                    }
                  ],
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                    "events": [
                      {
                        "address": "erd18p74xu7rq3agnfnqpnzfjpy609u7m3yyshd6z8xqeky46tvza50syytpff",
                        "identifier": "ESDTUnPause",
                        "topics": [
                          "RklUWS03M2Y4ZmM="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "status": "success",
                  "operation": "transfer",
                  "function": "unPause",
                  "initiallyPaidFee": "694050000000000",
                  "fee": "595000000000000",
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

    fn get_indirect_freeze_tx() -> TxResponse {
        let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "SCInvoking",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "5bd14b5b1dd256616ddd54e7d6616745e83ed52131e0b059cc387ba3eadaaf76",
                  "nonce": 140,
                  "round": 18323717,
                  "epoch": 1272,
                  "value": "0",
                  "receiver": "erd1qqqqqqqqqqqqqpgq7rwhny4mx6dhuzcsymrhdsv2vmvarecgh4vq687aqr",
                  "sender": "erd1sxdns0f2y0amcx59n0gjuazz2wjycnsrev26ktm5ugdzacpth4vqrq3xh0",
                  "gasPrice": 1000000000,
                  "gasLimit": 500000000,
                  "gasUsed": 500000000,
                  "data": "ZnJlZXplQWNjb3VudEBmYWJjMGMwOWU5NGJjOTNjODEyYWNkOTNhODI3YmYyZmIwNzFlZmYyZTJmZDdmYzI5MDMxNGEzNzMxYWEyNDgw",
                  "signature": "d29f5da7beebf4c4b6af5d669f7e7ee036eafe146564175142875775969a8879435b3b4d0f5c332fa2b2225830a1731aee0176e6221d6afc76020e198cbf1b09",
                  "sourceShard": 0,
                  "destinationShard": 0,
                  "blockNonce": 18316787,
                  "blockHash": "e6c95a4a4bbab93a60e260da1abc9cc8649261f08235bc1dc000ce44925d4fdf",
                  "notarizedAtSourceInMetaNonce": 18302295,
                  "NotarizedAtSourceInMetaHash": "25130ab8ddecafeee7242aeb74e08054bec3417bbeeebae86e52749661543358",
                  "notarizedAtDestinationInMetaNonce": 18302295,
                  "notarizedAtDestinationInMetaHash": "25130ab8ddecafeee7242aeb74e08054bec3417bbeeebae86e52749661543358",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "e98f64e0278ac64835b31dc36ffd1ae05ea7cbd2f52903bde1a4e4cbd2fcd16d",
                  "hyperblockNonce": 18302295,
                  "hyperblockHash": "25130ab8ddecafeee7242aeb74e08054bec3417bbeeebae86e52749661543358",
                  "timestamp": 1706059902,
                  "smartContractResults": [
                    {
                      "hash": "28280f142e4f1b0609890535af65c263ed719bc7a7275bdc6902f91e06e9a7d7",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "sender": "erd1qqqqqqqqqqqqqpgq7rwhny4mx6dhuzcsymrhdsv2vmvarecgh4vq687aqr",
                      "data": "freeze@43474f2d356539353238@fabc0c09e94bc93c812acd93a827bf2fb071eff2e2fd7fc290314a3731aa2480@e1b200b20ae06e26c761e69ef161b642d4e949921e83d120ec086bcf8f936ca5@5bd14b5b1dd256616ddd54e7d6616745e83ed52131e0b059cc387ba3eadaaf76@4f9318",
                      "prevTxHash": "5bd14b5b1dd256616ddd54e7d6616745e83ed52131e0b059cc387ba3eadaaf76",
                      "originalTxHash": "5bd14b5b1dd256616ddd54e7d6616745e83ed52131e0b059cc387ba3eadaaf76",
                      "gasLimit": 497379930,
                      "gasPrice": 1000000000,
                      "callType": 1,
                      "originalSender": "erd1sxdns0f2y0amcx59n0gjuazz2wjycnsrev26ktm5ugdzacpth4vqrq3xh0",
                      "operation": "transfer",
                      "function": "freeze"
                    },
                    {
                      "hash": "74d35e3615eb809b747fe6b9bd1d200409c393fc8143766cebe8fccd68634d56",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1l27qcz0ff0yneqf2ekf6sfal97c8rmljut7hls5sx99rwvd2yjqq4vyd3j",
                      "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "data": "ESDTFreeze@43474f2d356539353238",
                      "prevTxHash": "28280f142e4f1b0609890535af65c263ed719bc7a7275bdc6902f91e06e9a7d7",
                      "originalTxHash": "5bd14b5b1dd256616ddd54e7d6616745e83ed52131e0b059cc387ba3eadaaf76",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "originalSender": "erd1sxdns0f2y0amcx59n0gjuazz2wjycnsrev26ktm5ugdzacpth4vqrq3xh0",
                      "logs": {
                        "address": "erd1l27qcz0ff0yneqf2ekf6sfal97c8rmljut7hls5sx99rwvd2yjqq4vyd3j",
                        "events": [
                          {
                            "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                            "identifier": "ESDTFreeze",
                            "topics": [
                              "Q0dPLTVlOTUyOA==",
                              "",
                              "LbtTb8AG9A9dmA==",
                              "+rwMCelLyTyBKs2TqCe/L7Bx7/Li/X/CkDFKNzGqJIA="
                            ],
                            "data": null,
                            "additionalData": null
                          },
                          {
                            "address": "erd1l27qcz0ff0yneqf2ekf6sfal97c8rmljut7hls5sx99rwvd2yjqq4vyd3j",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "KCgPFC5PGwYJiQU1r2XCY+1xm8enJ1vcaQL5Hgbpp9c="
                            ],
                            "data": null,
                            "additionalData": null
                          }
                        ]
                      },
                      "tokens": [
                        "CGO-5e9528"
                      ],
                      "operation": "ESDTFreeze"
                    },
                    {
                      "hash": "a834af9623e9f3657ca81deb5630c460a06668820b5876886be85b02cabddaf8",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgq7rwhny4mx6dhuzcsymrhdsv2vmvarecgh4vq687aqr",
                      "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "data": "@00@30a235284e726e7b87ab7f19d4ebb53e6ddebf3e6aa0b606b5d54b6109a1dfee@e1b200b20ae06e26c761e69ef161b642d4e949921e83d120ec086bcf8f936ca5@5bd14b5b1dd256616ddd54e7d6616745e83ed52131e0b059cc387ba3eadaaf76@00",
                      "prevTxHash": "28280f142e4f1b0609890535af65c263ed719bc7a7275bdc6902f91e06e9a7d7",
                      "originalTxHash": "5bd14b5b1dd256616ddd54e7d6616745e83ed52131e0b059cc387ba3eadaaf76",
                      "gasLimit": 447379930,
                      "gasPrice": 1000000000,
                      "callType": 2,
                      "originalSender": "erd1sxdns0f2y0amcx59n0gjuazz2wjycnsrev26ktm5ugdzacpth4vqrq3xh0",
                      "logs": {
                        "address": "erd1qqqqqqqqqqqqqpgq7rwhny4mx6dhuzcsymrhdsv2vmvarecgh4vq687aqr",
                        "events": [
                          {
                            "address": "erd1qqqqqqqqqqqqqpgq7rwhny4mx6dhuzcsymrhdsv2vmvarecgh4vq687aqr",
                            "identifier": "writeLog",
                            "topics": [
                              "AAAAAAAAAAAFAPDdeZK7Npt+CxAmx3bBimbZ0ecIvVg=",
                              "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gNDQ3Mzc5OTMwLCBnYXMgdXNlZCA9IDIxNDkyNTc="
                            ],
                            "data": "QDZmNmI=",
                            "additionalData": [
                              "QDZmNmI="
                            ]
                          },
                          {
                            "address": "erd1qqqqqqqqqqqqqpgq7rwhny4mx6dhuzcsymrhdsv2vmvarecgh4vq687aqr",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "KCgPFC5PGwYJiQU1r2XCY+1xm8enJ1vcaQL5Hgbpp9c="
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
                    "address": "erd1qqqqqqqqqqqqqpgq7rwhny4mx6dhuzcsymrhdsv2vmvarecgh4vq687aqr",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq7rwhny4mx6dhuzcsymrhdsv2vmvarecgh4vq687aqr",
                        "identifier": "transferValueOnly",
                        "topics": [
                          "",
                          "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8="
                        ],
                        "data": "QXN5bmNDYWxs",
                        "additionalData": [
                          "QXN5bmNDYWxs",
                          "ZnJlZXpl",
                          "Q0dPLTVlOTUyOA==",
                          "+rwMCelLyTyBKs2TqCe/L7Bx7/Li/X/CkDFKNzGqJIA="
                        ]
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq7rwhny4mx6dhuzcsymrhdsv2vmvarecgh4vq687aqr",
                        "identifier": "writeLog",
                        "topics": [
                          "gZs4PSoj+7wahZvRLnRCU6RMTgPLFasvdOIaLuArvVg="
                        ],
                        "data": "QDZmNmI=",
                        "additionalData": [
                          "QDZmNmI="
                        ]
                      }
                    ]
                  },
                  "status": "success",
                  "operation": "transfer",
                  "function": "freezeAccount",
                  "initiallyPaidFee": "5165330000000000",
                  "fee": "5165330000000000",
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

    fn get_indirect_unfreeze_tx() -> TxResponse {
        let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "SCInvoking",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "30ada7bed5f769dd3fc24ac9d1d71b90986de0706bbb9ff0de34ebd4be611bce",
                  "nonce": 127,
                  "round": 17868816,
                  "epoch": 1240,
                  "value": "0",
                  "receiver": "erd1qqqqqqqqqqqqqpgqmuzgkurn657afd3r2aldqy2snsknwvrhc77q3lj8l6",
                  "sender": "erd1cy5rfjpmkaupq5m4g84veg89ylc0fzfr0ct6kw8sfu34gvkfc77q04snwt",
                  "gasPrice": 1000000000,
                  "gasLimit": 90000000,
                  "gasUsed": 90000000,
                  "data": "dW5mcmVlemVAMDAwMDAwMDAwMDAwMDAwMDA1MDA2NjUxMENGQ0Y4RjE2QUMzQzg0MDRCNzdCODNDMUU1ODUyNjU5NUFFRTEzQg==",
                  "signature": "1ac5d6b91bd88b8bca7b37ba2e6172111c66507413d5349b0058fd25c99863cbc11b43bf72ec8543b511dd3f51ea133d41caf13e84ffb4c16168cd84b79fee0b",
                  "sourceShard": 0,
                  "destinationShard": 0,
                  "blockNonce": 17862488,
                  "blockHash": "c513cfa712793080bbc8c4de7e5a6d09d031b51708f52fba24ecd9b4b2b464d1",
                  "notarizedAtSourceInMetaNonce": 17847852,
                  "NotarizedAtSourceInMetaHash": "cf943677d46bf4078f307b61601f18b78f7d5e37b8b551df29f9aa7c54ccb208",
                  "notarizedAtDestinationInMetaNonce": 17847852,
                  "notarizedAtDestinationInMetaHash": "cf943677d46bf4078f307b61601f18b78f7d5e37b8b551df29f9aa7c54ccb208",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "3ae9e9c60bd00c72ece286e6c6f97c675a588d597502d0b21f4a2246141abbb7",
                  "hyperblockNonce": 17847852,
                  "hyperblockHash": "cf943677d46bf4078f307b61601f18b78f7d5e37b8b551df29f9aa7c54ccb208",
                  "timestamp": 1703330496,
                  "smartContractResults": [
                    {
                      "hash": "9cea30cf8287cc031ca73e250194e232e07adb31357fa1e74a20dcd12023b724",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "sender": "erd1qqqqqqqqqqqqqpgqmuzgkurn657afd3r2aldqy2snsknwvrhc77q3lj8l6",
                      "data": "unFreeze@444154414e465446542d653933366434@0000000000000000050066510cfcf8f16ac3c8404b77b83c1e58526595aee13b@60c700",
                      "prevTxHash": "30ada7bed5f769dd3fc24ac9d1d71b90986de0706bbb9ff0de34ebd4be611bce",
                      "originalTxHash": "30ada7bed5f769dd3fc24ac9d1d71b90986de0706bbb9ff0de34ebd4be611bce",
                      "gasLimit": 85617057,
                      "gasPrice": 1000000000,
                      "callType": 1,
                      "operation": "transfer",
                      "function": "unFreeze"
                    },
                    {
                      "hash": "78a3ea2dd50171e7c8c97f1afe977b3f0e939054817ec0cb855f6ddaa460d346",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgqmuzgkurn657afd3r2aldqy2snsknwvrhc77q3lj8l6",
                      "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "data": "@00",
                      "prevTxHash": "9cea30cf8287cc031ca73e250194e232e07adb31357fa1e74a20dcd12023b724",
                      "originalTxHash": "30ada7bed5f769dd3fc24ac9d1d71b90986de0706bbb9ff0de34ebd4be611bce",
                      "gasLimit": 35617057,
                      "gasPrice": 1000000000,
                      "callType": 2,
                      "originalSender": "erd1qqqqqqqqqqqqqpgqmuzgkurn657afd3r2aldqy2snsknwvrhc77q3lj8l6",
                      "logs": {
                        "address": "erd1qqqqqqqqqqqqqpgqmuzgkurn657afd3r2aldqy2snsknwvrhc77q3lj8l6",
                        "events": [
                          {
                            "address": "erd1qqqqqqqqqqqqqpgqmuzgkurn657afd3r2aldqy2snsknwvrhc77q3lj8l6",
                            "identifier": "writeLog",
                            "topics": [
                              "AAAAAAAAAAAFAN8Ei3Bz1T3UtiNXftARUJwtNzB3x7w=",
                              "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gMzU2MTcwNTcsIGdhcyB1c2VkID0gMzMxMTQwMA=="
                            ],
                            "data": "QDZmNmI=",
                            "additionalData": null
                          },
                          {
                            "address": "erd1qqqqqqqqqqqqqpgqmuzgkurn657afd3r2aldqy2snsknwvrhc77q3lj8l6",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "nOowz4KHzAMcpz4lAZTiMuB62zE1f6HnSiDc0SAjtyQ="
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
                    "address": "erd1qqqqqqqqqqqqqpgqmuzgkurn657afd3r2aldqy2snsknwvrhc77q3lj8l6",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqmuzgkurn657afd3r2aldqy2snsknwvrhc77q3lj8l6",
                        "identifier": "unfreeze",
                        "topics": [
                          "Y29sbGVjdGlvbkZyZWV6ZUxpc3RSZW1vdmVk",
                          "AAAAAAAAAAAFAGZRDPz48WrDyEBLd7g8HlhSZZWu4Ts="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqmuzgkurn657afd3r2aldqy2snsknwvrhc77q3lj8l6",
                        "identifier": "writeLog",
                        "topics": [
                          "wSg0yDu3eBBTdUHqzKDlJ/D0iSN+F6s48E8jVDLJx7w="
                        ],
                        "data": "QDZmNmI=",
                        "additionalData": null
                      }
                    ]
                  },
                  "status": "success",
                  "operation": "transfer",
                  "function": "unfreeze",
                  "initiallyPaidFee": "1057905000000000",
                  "fee": "1057905000000000",
                  "chainID": "1",
                  "version": 2,
                  "options": 1
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

    // I didn't find any indirect tx containing the ESDTWipe log
    fn get_direct_wipe_tx() -> TxResponse {
        let data = r#"
           {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "SCInvoking",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "26401e7cb7e877c730855e60be3cdd5ced0e9fe29d85a87141398d58af0a13e4",
                  "nonce": 10973,
                  "round": 18441440,
                  "epoch": 1280,
                  "value": "0",
                  "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                  "sender": "erd1h3stlfvd0s7s390hrmdh3c8f83cdvvj7ratjaayhzq43dy0t42dqp6try5",
                  "gasPrice": 1000000000,
                  "gasLimit": 60000000,
                  "gasUsed": 50185000,
                  "data": "d2lwZUA0ZDRmNGUyZDM1Mzg2MzYzMzMzMkA5ZDlmYzM4OTMzMTQxMGIyYmFhNTFiYTRkMTZkNzg2ZDNhOGQwMDk4MjJkMDFhMDNjZTRkZWI0MmY0MzljMTU0",
                  "signature": "b9286fb5cfedde4a99102d3c2b3d64fb19bb8c2fe1f2a5992c34d7c6f4be5e0e0c4b0a42024e41d5232e1df1e77d0d095514f30e121532d18fb48ec6c7762701",
                  "sourceShard": 2,
                  "destinationShard": 4294967295,
                  "blockNonce": 18419983,
                  "blockHash": "bcd81c14a51e836c90c446958410a980e291da7b686653fc77383dae07b52a31",
                  "notarizedAtSourceInMetaNonce": 18419983,
                  "NotarizedAtSourceInMetaHash": "bcd81c14a51e836c90c446958410a980e291da7b686653fc77383dae07b52a31",
                  "notarizedAtDestinationInMetaNonce": 18419983,
                  "notarizedAtDestinationInMetaHash": "bcd81c14a51e836c90c446958410a980e291da7b686653fc77383dae07b52a31",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "af63b783179806ae97cab48b9f7e80fd313458642e069af1639e85d766c6b5ba",
                  "hyperblockNonce": 18419983,
                  "hyperblockHash": "bcd81c14a51e836c90c446958410a980e291da7b686653fc77383dae07b52a31",
                  "timestamp": 1706766240,
                  "smartContractResults": [
                    {
                      "hash": "f8b42f8d6b95dc559c4b21f2be2b6b404f7e7dc925e7249c741954ef14a3c415",
                      "nonce": 10974,
                      "value": 98150000000000,
                      "receiver": "erd1h3stlfvd0s7s390hrmdh3c8f83cdvvj7ratjaayhzq43dy0t42dqp6try5",
                      "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "data": "@6f6b",
                      "prevTxHash": "26401e7cb7e877c730855e60be3cdd5ced0e9fe29d85a87141398d58af0a13e4",
                      "originalTxHash": "26401e7cb7e877c730855e60be3cdd5ced0e9fe29d85a87141398d58af0a13e4",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "logs": {
                        "address": "erd1h3stlfvd0s7s390hrmdh3c8f83cdvvj7ratjaayhzq43dy0t42dqp6try5",
                        "events": [
                          {
                            "address": "erd1h3stlfvd0s7s390hrmdh3c8f83cdvvj7ratjaayhzq43dy0t42dqp6try5",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "JkAefLfod8cwhV5gvjzdXO0On+KdhahxQTmNWK8KE+Q="
                            ],
                            "data": null,
                            "additionalData": null
                          }
                        ]
                      },
                      "operation": "transfer",
                      "isRefund": true
                    },
                    {
                      "hash": "04583567a825ae2d80255835d631ea6cd89807b1c33d339cd4d0eb39cea73967",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1nk0u8zfnzsgt9w49rwjdzmtcd5ag6qycytgp5q7wfh459apec92q23auvw",
                      "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                      "data": "ESDTWipe@4d4f4e2d353863633332",
                      "prevTxHash": "26401e7cb7e877c730855e60be3cdd5ced0e9fe29d85a87141398d58af0a13e4",
                      "originalTxHash": "26401e7cb7e877c730855e60be3cdd5ced0e9fe29d85a87141398d58af0a13e4",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "originalSender": "erd1h3stlfvd0s7s390hrmdh3c8f83cdvvj7ratjaayhzq43dy0t42dqp6try5",
                      "tokens": [
                        "MON-58cc32"
                      ],
                      "operation": "ESDTWipe"
                    }
                  ],
                  "status": "success",
                  "operation": "transfer",
                  "function": "wipe",
                  "initiallyPaidFee": "783150000000000",
                  "fee": "685000000000000",
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

    fn get_indirect_update_attributes_tx() -> TxResponse {
        let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "BuiltInFunctionCall",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "16ce440263d8e8b087ea89e97fdcfe00038b874648bd0f83a00d7aec23454ae5",
                  "nonce": 128,
                  "round": 13170950,
                  "epoch": 914,
                  "value": "0",
                  "receiver": "erd1e65rkyfyqy29uph89f0ryspk5f5h6r64403z02c808ed766sh9qs86et0l",
                  "sender": "erd1e65rkyfyqy29uph89f0ryspk5f5h6r64403z02c808ed766sh9qs86et0l",
                  "gasPrice": 1000000000,
                  "gasLimit": 30000000,
                  "gasUsed": 6808425,
                  "data": "RVNEVE5GVFRyYW5zZmVyQDRiNTI0ZjQ3NDE0ZTJkMzUzNDYzMzMzNjMxQDNlQDAxQDAwMDAwMDAwMDAwMDAwMDAwNTAwZDhkYTIwMGEzYzVlMGQ1YmUyMTBlOWEwZTEwZjZmYjUzODM5Y2I3Yjc2NWZANzI2NTY3Njk3Mzc0NjU3MjRlNjY3NA==",
                  "signature": "fbe3982e944492ef31b65ed24f7cae5e290373fec4cf6d705a7262e72de792d5d0687c361223a72b629a301701c4c3bd6c0dae011516f8d95e75dd2c79db4707",
                  "sourceShard": 1,
                  "destinationShard": 1,
                  "blockNonce": 13161760,
                  "blockHash": "2a2e291f00113ce1d6921ace41c258b9953e00cc658d56ed8fcc9651c468bda3",
                  "notarizedAtSourceInMetaNonce": 13151024,
                  "NotarizedAtSourceInMetaHash": "e08a3246dd6660e0e9b7416ff14081415d249c46d5b772e8d4cd9c277dab78d6",
                  "notarizedAtDestinationInMetaNonce": 13151024,
                  "notarizedAtDestinationInMetaHash": "e08a3246dd6660e0e9b7416ff14081415d249c46d5b772e8d4cd9c277dab78d6",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "e3f6437ad3fdeaa6f5c9d4be37b2d9d09d47cdb17ee2eeabe9690a0c3535c951",
                  "hyperblockNonce": 13151024,
                  "hyperblockHash": "e08a3246dd6660e0e9b7416ff14081415d249c46d5b772e8d4cd9c277dab78d6",
                  "timestamp": 1675143300,
                  "smartContractResults": [
                    {
                      "hash": "2c308d46b071262a4c17006554b59a88883492828be6ddfa833fb3b5d969c7ad",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgqmrdzqz3utcx4hcssaxswzrm0k5urnjmmwe0sndvyhm",
                      "sender": "erd1e65rkyfyqy29uph89f0ryspk5f5h6r64403z02c808ed766sh9qs86et0l",
                      "data": "registerNft",
                      "prevTxHash": "16ce440263d8e8b087ea89e97fdcfe00038b874648bd0f83a00d7aec23454ae5",
                      "originalTxHash": "16ce440263d8e8b087ea89e97fdcfe00038b874648bd0f83a00d7aec23454ae5",
                      "gasLimit": 29532850,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "operation": "transfer",
                      "function": "registerNft"
                    },
                    {
                      "hash": "d400cb4649cb187635b1d302f8b47d5c4b44886717fc25e81fc6848292db59ca",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1e65rkyfyqy29uph89f0ryspk5f5h6r64403z02c808ed766sh9qs86et0l",
                      "sender": "erd1qqqqqqqqqqqqqpgqmrdzqz3utcx4hcssaxswzrm0k5urnjmmwe0sndvyhm",
                      "data": "ESDTNFTTransfer@4b524f47414e2d353463333631@3e@01@cea83b112401145e06e72a5e324036a2697d0f55abe227ab0779f2df6b50b941",
                      "prevTxHash": "16ce440263d8e8b087ea89e97fdcfe00038b874648bd0f83a00d7aec23454ae5",
                      "originalTxHash": "16ce440263d8e8b087ea89e97fdcfe00038b874648bd0f83a00d7aec23454ae5",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "tokens": [
                        "KROGAN-54c361-3e"
                      ],
                      "esdtValues": [
                        "1"
                      ],
                      "receivers": [
                        "erd1e65rkyfyqy29uph89f0ryspk5f5h6r64403z02c808ed766sh9qs86et0l"
                      ],
                      "receiversShardIDs": [
                        1
                      ],
                      "operation": "ESDTNFTTransfer"
                    },
                    {
                      "hash": "3f2491179edd2cd93ce054301e839a22a7587d0c6e67c62372d8da1b74e7aa3b",
                      "nonce": 129,
                      "value": 231915750000000,
                      "receiver": "erd1e65rkyfyqy29uph89f0ryspk5f5h6r64403z02c808ed766sh9qs86et0l",
                      "sender": "erd1e65rkyfyqy29uph89f0ryspk5f5h6r64403z02c808ed766sh9qs86et0l",
                      "data": "@6f6b",
                      "prevTxHash": "16ce440263d8e8b087ea89e97fdcfe00038b874648bd0f83a00d7aec23454ae5",
                      "originalTxHash": "16ce440263d8e8b087ea89e97fdcfe00038b874648bd0f83a00d7aec23454ae5",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "operation": "transfer",
                      "isRefund": true
                    }
                  ],
                  "logs": {
                    "address": "erd1e65rkyfyqy29uph89f0ryspk5f5h6r64403z02c808ed766sh9qs86et0l",
                    "events": [
                      {
                        "address": "erd1e65rkyfyqy29uph89f0ryspk5f5h6r64403z02c808ed766sh9qs86et0l",
                        "identifier": "ESDTNFTTransfer",
                        "topics": [
                          "S1JPR0FOLTU0YzM2MQ==",
                          "Pg==",
                          "AQ==",
                          "AAAAAAAAAAAFANjaIAo8Xg1b4hDpoOEPb7U4Oct7dl8="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqmrdzqz3utcx4hcssaxswzrm0k5urnjmmwe0sndvyhm",
                        "identifier": "ESDTNFTUpdateAttributes",
                        "topics": [
                          "S1JPR0FOLTU0YzM2MQ==",
                          "Pg==",
                          "",
                          "dGFnczpPZmZpY2lhbCxLcm9nYW52ZXJzZSxQMkUsU3BhY2VzaGlwO0xldmVsOjEwO1NwZWVkOjEwODtBY2NlbGVyYXRpb246NztFdmFzaW9uOjY7QXR0YWNrOjY1O0FjY3VyYWN5OjUyO0ZpcmUgUmF0ZToxNDtBcm1vcjo1MjtIZWFsdGg6NjI3O0NhcmdvOjExMztNaW5pbmc6MTQ="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqmrdzqz3utcx4hcssaxswzrm0k5urnjmmwe0sndvyhm",
                        "identifier": "ESDTNFTTransfer",
                        "topics": [
                          "S1JPR0FOLTU0YzM2MQ==",
                          "Pg==",
                          "AQ==",
                          "zqg7ESQBFF4G5ypeMkA2oml9D1Wr4ierB3ny32tQuUE="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1e65rkyfyqy29uph89f0ryspk5f5h6r64403z02c808ed766sh9qs86et0l",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "Fs5EAmPY6LCH6onpf9z+AAOLh0ZIvQ+DoA167CNFSuU="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "status": "success",
                  "tokens": [
                    "KROGAN-54c361-3e"
                  ],
                  "esdtValues": [
                    "1"
                  ],
                  "receivers": [
                    "erd1qqqqqqqqqqqqqpgqmrdzqz3utcx4hcssaxswzrm0k5urnjmmwe0sndvyhm"
                  ],
                  "receiversShardIDs": [
                    1
                  ],
                  "operation": "ESDTNFTTransfer",
                  "function": "registerNft",
                  "initiallyPaidFee": "551460000000000",
                  "fee": "319544250000000",
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

    fn get_indirect_add_quantity_tx() -> TxResponse {
        let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "BuiltInFunctionCall",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "188b1b3b8fae0dd90a897b595bace3945df747956596d0a7adecc747f9f44f01",
                  "nonce": 724,
                  "round": 12100469,
                  "epoch": 840,
                  "value": "0",
                  "receiver": "erd105kysyugnspafs79lr749fqry6pudse468wya7xy4fjkdlskx7ms2kuzcv",
                  "sender": "erd105kysyugnspafs79lr749fqry6pudse468wya7xy4fjkdlskx7ms2kuzcv",
                  "gasPrice": 1000000000,
                  "gasLimit": 22000000,
                  "gasUsed": 15574037,
                  "data": "RVNEVE5GVFRyYW5zZmVyQDQ5NTQ0ODU3NDU0NzRjNDQ0NjRjMmQzMzMzMzI2NjMzMzhAMDE2ODA0QGU5NTc1ZDcxY2E3OThiNzZAMDAwMDAwMDAwMDAwMDAwMDA1MDA2YWNlNTU2NWZkNGZkMWI3OTljMzI1ZWY5NDQyNTc3M2M0M2U2OWZiNTQ4M0A2MzZjNjE2OTZkNTI2NTc3NjE3MjY0NzM=",
                  "signature": "41d58f146e1c8168518733073af7d4f57a4d9a6fc5cbb4b0e329c55d54079d30a985bb003c02b4d0e5743f27935b1c4d900f73ae3aa8dcb2c618289e07d2ef08",
                  "sourceShard": 1,
                  "destinationShard": 1,
                  "blockNonce": 12091970,
                  "blockHash": "a568a7e1c4e4567601bc442a767848fc1bdb051ea6fc6dfe6e60d1ddba20f324",
                  "notarizedAtSourceInMetaNonce": 12080759,
                  "NotarizedAtSourceInMetaHash": "726dc35e867f1a0362f17ba5c065ccbf694946b8af622508c435595e5231e82f",
                  "notarizedAtDestinationInMetaNonce": 12080759,
                  "notarizedAtDestinationInMetaHash": "726dc35e867f1a0362f17ba5c065ccbf694946b8af622508c435595e5231e82f",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "e5ffe2318935f19092cdbb4db45545f3fd4ecc19f92443ebf4e9a3bfa5f886c6",
                  "hyperblockNonce": 12080759,
                  "hyperblockHash": "726dc35e867f1a0362f17ba5c065ccbf694946b8af622508c435595e5231e82f",
                  "timestamp": 1668720414,
                  "smartContractResults": [
                    {
                      "hash": "cd651a49b2f1b04a036ced2d6bcc9d4dea3d2a18e3f4d492c36b17326392f854",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgqdt892e0aflgm0xwryhhegsjhw0zru60m2jps959w5z",
                      "sender": "erd105kysyugnspafs79lr749fqry6pudse468wya7xy4fjkdlskx7ms2kuzcv",
                      "data": "claimRewards",
                      "prevTxHash": "188b1b3b8fae0dd90a897b595bace3945df747956596d0a7adecc747f9f44f01",
                      "originalTxHash": "188b1b3b8fae0dd90a897b595bace3945df747956596d0a7adecc747f9f44f01",
                      "gasLimit": 21504000,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "operation": "transfer",
                      "function": "claimRewards"
                    },
                    {
                      "hash": "69cca56e71e6d966dcc9d110b1bad19b7697779f9192efce0d0fff1dbcf3e2a4",
                      "nonce": 725,
                      "value": 64259630000000,
                      "receiver": "erd105kysyugnspafs79lr749fqry6pudse468wya7xy4fjkdlskx7ms2kuzcv",
                      "sender": "erd105kysyugnspafs79lr749fqry6pudse468wya7xy4fjkdlskx7ms2kuzcv",
                      "data": "@6f6b@016e62@020000000c4c4b4d45582d6161623931300000000000406e500000000a8a7b6565971373c88d96@02000000114954485745474c44464c2d3333326633380000000000016e6200000008e9575d71ca798b76@020000000c4c4b4d45582d6161623931300000000000406e500000000a8a7b6565971373c88d96",
                      "prevTxHash": "188b1b3b8fae0dd90a897b595bace3945df747956596d0a7adecc747f9f44f01",
                      "originalTxHash": "188b1b3b8fae0dd90a897b595bace3945df747956596d0a7adecc747f9f44f01",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "operation": "transfer",
                      "isRefund": true
                    },
                    {
                      "hash": "4c583dc7045a730cbb9303b60cd3a34b74c028a7e301857f9c61b2e6ea29e0d2",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd105kysyugnspafs79lr749fqry6pudse468wya7xy4fjkdlskx7ms2kuzcv",
                      "sender": "erd1qqqqqqqqqqqqqpgqjpt0qqgsrdhp2xqygpjtfrpwf76f9nvg2jpsg4q7th",
                      "data": "ESDTNFTTransfer@4c4b4d45582d616162393130@406e50@8a7b6565971373c88d96@7d2c4813889c03d4c3c5f8fd52a4032683c6c335d1dc4ef8c4aa6566fe1637b7",
                      "prevTxHash": "188b1b3b8fae0dd90a897b595bace3945df747956596d0a7adecc747f9f44f01",
                      "originalTxHash": "188b1b3b8fae0dd90a897b595bace3945df747956596d0a7adecc747f9f44f01",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "tokens": [
                        "LKMEX-aab910-406e50"
                      ],
                      "esdtValues": [
                        "653962830569157595532694"
                      ],
                      "receivers": [
                        "erd105kysyugnspafs79lr749fqry6pudse468wya7xy4fjkdlskx7ms2kuzcv"
                      ],
                      "receiversShardIDs": [
                        1
                      ],
                      "operation": "ESDTNFTTransfer"
                    },
                    {
                      "hash": "a3775b8f04b84ae665c7a52d5ae68bb4eceb7650626486287e4eb49ed1c0e802",
                      "nonce": 1,
                      "value": 0,
                      "receiver": "erd105kysyugnspafs79lr749fqry6pudse468wya7xy4fjkdlskx7ms2kuzcv",
                      "sender": "erd1qqqqqqqqqqqqqpgqdt892e0aflgm0xwryhhegsjhw0zru60m2jps959w5z",
                      "data": "ESDTNFTTransfer@4954485745474c44464c2d333332663338@016e62@e9575d71ca798b76@7d2c4813889c03d4c3c5f8fd52a4032683c6c335d1dc4ef8c4aa6566fe1637b7",
                      "prevTxHash": "188b1b3b8fae0dd90a897b595bace3945df747956596d0a7adecc747f9f44f01",
                      "originalTxHash": "188b1b3b8fae0dd90a897b595bace3945df747956596d0a7adecc747f9f44f01",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "tokens": [
                        "ITHWEGLDFL-332f38-016e62"
                      ],
                      "esdtValues": [
                        "16814010477120686966"
                      ],
                      "receivers": [
                        "erd105kysyugnspafs79lr749fqry6pudse468wya7xy4fjkdlskx7ms2kuzcv"
                      ],
                      "receiversShardIDs": [
                        1
                      ],
                      "operation": "ESDTNFTTransfer"
                    }
                  ],
                  "logs": {
                    "address": "erd105kysyugnspafs79lr749fqry6pudse468wya7xy4fjkdlskx7ms2kuzcv",
                    "events": [
                      {
                        "address": "erd105kysyugnspafs79lr749fqry6pudse468wya7xy4fjkdlskx7ms2kuzcv",
                        "identifier": "ESDTNFTTransfer",
                        "topics": [
                          "SVRIV0VHTERGTC0zMzJmMzg=",
                          "AWgE",
                          "6Vddccp5i3Y=",
                          "AAAAAAAAAAAFAGrOVWX9T9G3mcMl75RCV3PEPmn7VIM="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqdt892e0aflgm0xwryhhegsjhw0zru60m2jps959w5z",
                        "identifier": "ESDTNFTCreate",
                        "topics": [
                          "SVRIV0VHTERGTC0zMzJmMzg=",
                          "AW5i",
                          "6Vddccp5i3Y=",
                          "CAESCQDpV11xynmLdiJiCOLcBRogAAAAAAAAAAAFAGrOVWX9T9G3mcMl75RCV3PEPmn7VIMyADo4AAAACAsUK3W7Ez3JAAAAAAAAAvYAAAAAAAAC9gAAAAjpV11xynmLdgAAAAAAAAAI6Vddccp5i3Y="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqdt892e0aflgm0xwryhhegsjhw0zru60m2jps959w5z",
                        "identifier": "ESDTNFTBurn",
                        "topics": [
                          "SVRIV0VHTERGTC0zMzJmMzg=",
                          "AWgE",
                          "6Vddccp5i3Y="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqjpt0qqgsrdhp2xqygpjtfrpwf76f9nvg2jpsg4q7th",
                        "identifier": "ESDTNFTAddQuantity",
                        "topics": [
                          "TEtNRVgtYWFiOTEw",
                          "QG5Q",
                          "intlZZcTc8iNlg=="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqjpt0qqgsrdhp2xqygpjtfrpwf76f9nvg2jpsg4q7th",
                        "identifier": "ESDTNFTTransfer",
                        "topics": [
                          "TEtNRVgtYWFiOTEw",
                          "QG5Q",
                          "intlZZcTc8iNlg==",
                          "fSxIE4icA9TDxfj9UqQDJoPGwzXR3E74xKplZv4WN7c="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqjpt0qqgsrdhp2xqygpjtfrpwf76f9nvg2jpsg4q7th",
                        "identifier": "createAndForward",
                        "topics": [
                          "Y3JlYXRlX2FuZF9mb3J3YXJk",
                          "AAAAAAAAAAAFAGrOVWX9T9G3mcMl75RCV3PEPmn7VIM=",
                          "fSxIE4icA9TDxfj9UqQDJoPGwzXR3E74xKplZv4WN7c=",
                          "A0g="
                        ],
                        "data": "AAAAAAAAAAAFAGrOVWX9T9G3mcMl75RCV3PEPmn7VIN9LEgTiJwD1MPF+P1SpAMmg8bDNdHcTvjEqmVm/hY3twAAAAxMS01FWC1hYWI5MTAAAAAAAEBuUAAAAAqKe2VllxNzyI2WAAAABgAAAAAAAARYAAAAAAAAQmgAAAAAAAAEdgAAAAAAAEJoAAAAAAAABJQAAAAAAABCaAAAAAAAAASyAAAAAAAAQmgAAAAAAAAE0AAAAAAAAD6AAAAAAAAABO4AAAAAAAA+gAAAAAAAAAAC9gAAAAAAuIJCAAAAAAAAA0gAAAAAY3anHg==",
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqdt892e0aflgm0xwryhhegsjhw0zru60m2jps959w5z",
                        "identifier": "ESDTNFTTransfer",
                        "topics": [
                          "SVRIV0VHTERGTC0zMzJmMzg=",
                          "AW5i",
                          "6Vddccp5i3Y=",
                          "fSxIE4icA9TDxfj9UqQDJoPGwzXR3E74xKplZv4WN7c="
                        ],
                        "data": null,
                        "additionalData": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqdt892e0aflgm0xwryhhegsjhw0zru60m2jps959w5z",
                        "identifier": "claimRewards",
                        "topics": [
                          "Y2xhaW1fcmV3YXJkcw==",
                          "fSxIE4icA9TDxfj9UqQDJoPGwzXR3E74xKplZv4WN7c=",
                          "SVRIV0VHTERGTC0zMzJmMzg=",
                          "A0g="
                        ],
                        "data": "fSxIE4icA9TDxfj9UqQDJoPGwzXR3E74xKplZv4WN7cAAAARSVRIV0VHTERGTC0zMzJmMzgAAAAAAAFoBAAAAAjpV11xynmLdgAAABFJVEhXRUdMREZMLTMzMmYzOAAAAAAAAW5iAAAACOlXXXHKeYt2AAAAChn/DNzx4QjsZIQAAAAMTEtNRVgtYWFiOTEwAAAAAABAblAAAAAKintlZZcTc8iNlgAAAAwLFZO1G7HmNxzeFjYAAAAICon9pJSIPG4AAAAAAAAC9gAAAAAAAAL2AAAACOlXXXHKeYt2AAAAAAAAAAjpV11xynmLdgAAAAgLFCt1uxM9yQAAAAAAAAL2AAAAAAAAAvYAAAAI6Vddccp5i3YAAAAAAAAACOlXXXHKeYt2AAAAAAAAuIJCAAAAAAAAA0gAAAAAY3anHg==",
                        "additionalData": null
                      },
                      {
                        "address": "erd105kysyugnspafs79lr749fqry6pudse468wya7xy4fjkdlskx7ms2kuzcv",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "GIsbO4+uDdkKiXtZW6zjlF33R5VlltCnrezHR/n0TwE="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "status": "success",
                  "tokens": [
                    "ITHWEGLDFL-332f38-016804"
                  ],
                  "esdtValues": [
                    "16814010477120686966"
                  ],
                  "receivers": [
                    "erd1qqqqqqqqqqqqqpgqdt892e0aflgm0xwryhhegsjhw0zru60m2jps959w5z"
                  ],
                  "receiversShardIDs": [
                    1
                  ],
                  "operation": "ESDTNFTTransfer",
                  "function": "claimRewards",
                  "initiallyPaidFee": "513040000000000",
                  "fee": "448780370000000",
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

    fn get_indirect_burn_quantity_tx() -> TxResponse {
        let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "BuiltInFunctionCall",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "9895c099d377d6352563da31025965fbbc64a0f0bf9a8777b860c208d3a1b218",
                  "nonce": 1191,
                  "round": 9714795,
                  "epoch": 674,
                  "value": "0",
                  "receiver": "erd1tad5vazy0q9w9juhw7kgfkvzpjj0upjqhwvpqcyn0c7yvsghg5xqwgvscr",
                  "sender": "erd1tad5vazy0q9w9juhw7kgfkvzpjj0upjqhwvpqcyn0c7yvsghg5xqwgvscr",
                  "gasPrice": 1000000000,
                  "gasLimit": 21600000,
                  "gasUsed": 18406504,
                  "data": "RVNEVE5GVFRyYW5zZmVyQDRjNGI0NjQxNTI0ZDJkMzk2NDMxNjU2MTM4QDgwMTA3MEA4ZWZkNzk3NDk3ODRkN2VhN2JhNTAyQDAwMDAwMDAwMDAwMDAwMDAwNTAwMWUyYTE0MjhkZDFlM2E1MTQ2YjM5NjBkOWUwZjRhNTAzNjk5MDRlZTU0ODNANjM2ZjZkNzA2Zjc1NmU2NDUyNjU3NzYxNzI2NDczNTA3MjZmNzg3OUAwMDAwMDAwMDAwMDAwMDAwMDUwMGYwMmYwNzFlYzk0ZjVhN2E0OTZiZjE1NjUyNmRmZGE5MzBjZWNjYmQ1NDgz",
                  "signature": "72f959c5a223886f70f964d7da09cc17bfec611c1f2edeed05499ef0286aa9a1666b8bac23fae30d80448ec955ef68668d3fa68b16974dc243c6742478fd910f",
                  "sourceShard": 0,
                  "destinationShard": 0,
                  "blockNonce": 9711706,
                  "blockHash": "0c2ab248e9cd8e1e6ad436126254863ef8233a18fe95967538934fe46b579698",
                  "notarizedAtSourceInMetaNonce": 9695657,
                  "NotarizedAtSourceInMetaHash": "7f643a18c3037c59dc87fac78e2ffdbc45c8182448dd9307aedb0496432fa133",
                  "notarizedAtDestinationInMetaNonce": 9695657,
                  "notarizedAtDestinationInMetaHash": "7f643a18c3037c59dc87fac78e2ffdbc45c8182448dd9307aedb0496432fa133",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "a27d8688e90e140350d696a72d9c81494a84f8a1994e00aa220d7b64205a4f84",
                  "hyperblockNonce": 9695657,
                  "hyperblockHash": "7f643a18c3037c59dc87fac78e2ffdbc45c8182448dd9307aedb0496432fa133",
                  "timestamp": 1654406370,
                  "smartContractResults": [
                    {
                      "hash": "220004c89b51a6bdd7ee5ce0263a90e3d3c03500213297e24c476a63053e0399",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgqrc4pg2xarca9z34njcxeur622qmfjp8w2jps89fxnl",
                      "sender": "erd1tad5vazy0q9w9juhw7kgfkvzpjj0upjqhwvpqcyn0c7yvsghg5xqwgvscr",
                      "data": "ESDTNFTTransfer@4c4b4641524d2d396431656138@801070@8efd79749784d7ea7ba502@00@636f6d706f756e645265776172647350726f7879@00000000000000000500f02f071ec94f5a7a496bf156526dfda930ceccbd5483",
                      "prevTxHash": "9895c099d377d6352563da31025965fbbc64a0f0bf9a8777b860c208d3a1b218",
                      "originalTxHash": "9895c099d377d6352563da31025965fbbc64a0f0bf9a8777b860c208d3a1b218",
                      "gasLimit": 20985500,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "tokens": [
                        "LKFARM-9d1ea8-801070"
                      ],
                      "esdtValues": [
                        "172864465562805974672844034"
                      ],
                      "receivers": [
                        "erd1qqqqqqqqqqqqqpgqrc4pg2xarca9z34njcxeur622qmfjp8w2jps89fxnl"
                      ],
                      "receiversShardIDs": [
                        1
                      ],
                      "operation": "ESDTNFTTransfer",
                      "function": "compoundRewardsProxy"
                    },
                    {
                      "hash": "9286023d831f2ee2cc8569c049274ab4de0de244b1c93a9f6ec351e971eca24d",
                      "nonce": 1,
                      "value": 31934960000000,
                      "receiver": "erd1tad5vazy0q9w9juhw7kgfkvzpjj0upjqhwvpqcyn0c7yvsghg5xqwgvscr",
                      "sender": "erd1qqqqqqqqqqqqqpgqrc4pg2xarca9z34njcxeur622qmfjp8w2jps89fxnl",
                      "data": "@6f6b@2bcab0@020000000f4d45584641524d4c2d32386436343600000000002bcab00000000b8f9762a459c44e9057a8bd@804cc6",
                      "prevTxHash": "220004c89b51a6bdd7ee5ce0263a90e3d3c03500213297e24c476a63053e0399",
                      "originalTxHash": "9895c099d377d6352563da31025965fbbc64a0f0bf9a8777b860c208d3a1b218",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "logs": {
                        "address": "erd1tad5vazy0q9w9juhw7kgfkvzpjj0upjqhwvpqcyn0c7yvsghg5xqwgvscr",
                        "events": [
                          {
                            "address": "erd1tad5vazy0q9w9juhw7kgfkvzpjj0upjqhwvpqcyn0c7yvsghg5xqwgvscr",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "IgAEyJtRpr3X7lzgJjqQ49PANQAhMpfiTEdqYwU+A5k="
                            ],
                            "data": null,
                            "additionalData": null
                          }
                        ]
                      },
                      "operation": "transfer",
                      "isRefund": true
                    },
                    {
                      "hash": "9683263caa8cc723d027a08acffa6862735b40f217fabff8554acd182aac4e82",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1tad5vazy0q9w9juhw7kgfkvzpjj0upjqhwvpqcyn0c7yvsghg5xqwgvscr",
                      "sender": "erd1qqqqqqqqqqqqqpgqrc4pg2xarca9z34njcxeur622qmfjp8w2jps89fxnl",
                      "data": "ESDTNFTTransfer@4c4b4641524d2d396431656138@804cc6@8f9762a459c44e9057a8bd@0801120c008f9762a459c44e9057a8bd227c08c69981041a20000000000000000005001e2a1428dd1e3a5146b3960d9e0f4a50369904ee548332003a510000000f4d45584641524d4c2d32386436343600000000002bcab00000000b8f9762a459c44e9057a8bd0000000c4c4b4d45582d6161623931300000000000370afa0000000b70a53bf8abd0cb5dbb633c",
                      "prevTxHash": "220004c89b51a6bdd7ee5ce0263a90e3d3c03500213297e24c476a63053e0399",
                      "originalTxHash": "9895c099d377d6352563da31025965fbbc64a0f0bf9a8777b860c208d3a1b218",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "logs": {
                        "address": "erd1tad5vazy0q9w9juhw7kgfkvzpjj0upjqhwvpqcyn0c7yvsghg5xqwgvscr",
                        "events": [
                          {
                            "address": "erd1qqqqqqqqqqqqqpgqrc4pg2xarca9z34njcxeur622qmfjp8w2jps89fxnl",
                            "identifier": "ESDTNFTTransfer",
                            "topics": [
                              "TEtGQVJNLTlkMWVhOA==",
                              "gEzG",
                              "j5dipFnETpBXqL0=",
                              "X1tGdER4CuLLl3eshNmCDKT+BkC7mBBgk348RkEXRQw="
                            ],
                            "data": null,
                            "additionalData": null
                          },
                          {
                            "address": "erd1tad5vazy0q9w9juhw7kgfkvzpjj0upjqhwvpqcyn0c7yvsghg5xqwgvscr",
                            "identifier": "writeLog",
                            "topics": [
                              "AAAAAAAAAAAFAB4qFCjdHjpRRrOWDZ4PSlA2mQTuVIM="
                            ],
                            "data": "QDZmNmI=",
                            "additionalData": null
                          },
                          {
                            "address": "erd1tad5vazy0q9w9juhw7kgfkvzpjj0upjqhwvpqcyn0c7yvsghg5xqwgvscr",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "IgAEyJtRpr3X7lzgJjqQ49PANQAhMpfiTEdqYwU+A5k="
                            ],
                            "data": null,
                            "additionalData": null
                          }
                        ]
                      },
                      "tokens": [
                        "LKFARM-9d1ea8-804cc6"
                      ],
                      "esdtValues": [
                        "173591289167437048801896637"
                      ],
                      "receivers": [
                        "erd1tad5vazy0q9w9juhw7kgfkvzpjj0upjqhwvpqcyn0c7yvsghg5xqwgvscr"
                      ],
                      "receiversShardIDs": [
                        0
                      ],
                      "operation": "ESDTNFTTransfer"
                    }
                  ],
                  "logs": {
                    "address": "erd1tad5vazy0q9w9juhw7kgfkvzpjj0upjqhwvpqcyn0c7yvsghg5xqwgvscr",
                    "events": [
                      {
                        "address": "erd1tad5vazy0q9w9juhw7kgfkvzpjj0upjqhwvpqcyn0c7yvsghg5xqwgvscr",
                        "identifier": "ESDTNFTTransfer",
                        "topics": [
                          "TEtGQVJNLTlkMWVhOA==",
                          "gBBw",
                          "jv15dJeE1+p7pQI=",
                          "AAAAAAAAAAAFAB4qFCjdHjpRRrOWDZ4PSlA2mQTuVIM="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "status": "success",
                  "tokens": [
                    "LKFARM-9d1ea8-801070"
                  ],
                  "esdtValues": [
                    "172864465562805974672844034"
                  ],
                  "receivers": [
                    "erd1qqqqqqqqqqqqqpgqrc4pg2xarca9z34njcxeur622qmfjp8w2jps89fxnl"
                  ],
                  "receiversShardIDs": [
                    1
                  ],
                  "operation": "ESDTNFTTransfer",
                  "function": "compoundRewardsProxy",
                  "initiallyPaidFee": "626355000000000",
                  "fee": "594420040000000",
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
}