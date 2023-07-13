use multiversx_sc::types::H256;

use crate::{
    api::StaticApi,
    multiversx_sc::{
        codec::{CodecFrom, TopEncodeMulti},
        types::ContractCall,
    },
    scenario::model::{AddressValue, BytesValue, TxExpect, TxQuery},
};

use super::{format_expect, process_contract_call};

#[derive(Debug, Default, Clone)]
pub struct ScQueryStep {
    pub id: String,
    pub tx_id: Option<String>,
    pub explicit_tx_hash: Option<H256>,
    pub comment: Option<String>,
    pub tx: Box<TxQuery>,
    pub expect: Option<TxExpect>,
}

impl ScQueryStep {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to<A>(mut self, address: A) -> Self
    where
        AddressValue: From<A>,
    {
        self.tx.to = AddressValue::from(address);
        self
    }

    pub fn function(mut self, expr: &str) -> Self {
        self.tx.function = expr.to_string();
        self
    }

    pub fn argument(mut self, expr: &str) -> Self {
        self.tx.arguments.push(BytesValue::from(expr));
        self
    }

    pub fn expect(mut self, expect: TxExpect) -> Self {
        self.expect = Some(expect);
        self
    }

    /// Sets following fields based on the smart contract proxy:
    /// - "to"
    /// - "function"
    /// - "arguments"
    pub fn call<CC>(mut self, contract_call: CC) -> Self
    where
        CC: ContractCall<StaticApi>,
    {
        let (to_str, function, _, mandos_args) = process_contract_call(contract_call);
        self = self.to(to_str.as_str());
        self = self.function(function.as_str());
        for arg in mandos_args {
            self = self.argument(arg.as_str());
        }
        self
    }

    /// Sets following fields based on the smart contract proxy:
    /// - "to"
    /// - "function"
    /// - "arguments"
    /// - "expect"
    ///     - "out"
    ///     - "status" set to 0
    pub fn call_expect<CC, ExpectedResult>(
        mut self,
        contract_call: CC,
        expect_value: ExpectedResult,
    ) -> Self
    where
        CC: ContractCall<StaticApi>,
        ExpectedResult: CodecFrom<CC::OriginalResult> + TopEncodeMulti,
    {
        self = self.call(contract_call);
        self = self.expect(format_expect(expect_value));
        self
    }
}