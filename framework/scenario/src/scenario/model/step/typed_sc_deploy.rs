use std::marker::PhantomData;

use multiversx_sc::codec::PanicErrorHandler;

use crate::{
    multiversx_sc::{
        codec::{CodecFrom, TopEncodeMulti},
        types::{Address, CodeMetadata},
    },
    scenario_format::interpret_trait::InterpreterContext,
    scenario_model::{TxResponse, TxResponseStatus},
};

use crate::scenario::model::{AddressValue, BigUintValue, TxExpect, U64Value};

use super::ScDeployStep;

/// `ScDeployStep` with explicit return type.
#[derive(Default, Debug)]
pub struct TypedScDeploy<OriginalResult> {
    pub sc_deploy_step: ScDeployStep,
    _phantom: PhantomData<OriginalResult>,
}

impl<OriginalResult> TypedScDeploy<OriginalResult> {
    pub fn result<RequestedResult>(&self) -> Result<RequestedResult, TxResponseStatus>
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let mut raw_result = self.response().out.clone();
        Ok(
            RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler)
                .unwrap(),
        )
    }

    pub fn response(&self) -> &TxResponse {
        self.sc_deploy_step.response.as_ref().unwrap()
    }

    pub fn from<A>(mut self, address: A) -> Self
    where
        AddressValue: From<A>,
    {
        self.sc_deploy_step = self.sc_deploy_step.from(address);
        self
    }

    pub fn egld_value<A>(mut self, amount: A) -> Self
    where
        BigUintValue: From<A>,
    {
        self.sc_deploy_step = self.sc_deploy_step.egld_value(amount);
        self
    }

    pub fn code_metadata(mut self, code_metadata: CodeMetadata) -> Self {
        self.sc_deploy_step = self.sc_deploy_step.code_metadata(code_metadata);
        self
    }

    pub fn contract_code(mut self, expr: &str, context: &InterpreterContext) -> Self {
        self.sc_deploy_step = self.sc_deploy_step.contract_code(expr, context);
        self
    }

    pub fn gas_limit<V>(mut self, value: V) -> Self
    where
        U64Value: From<V>,
    {
        self.sc_deploy_step = self.sc_deploy_step.gas_limit(value);
        self
    }

    pub fn expect(mut self, expect: TxExpect) -> Self {
        self.sc_deploy_step = self.sc_deploy_step.expect(expect);
        self
    }
}

impl<OriginalResult> AsMut<ScDeployStep> for TypedScDeploy<OriginalResult> {
    fn as_mut(&mut self) -> &mut ScDeployStep {
        &mut self.sc_deploy_step
    }
}

impl<OriginalResult> From<TypedScDeploy<OriginalResult>> for ScDeployStep {
    fn from(typed: TypedScDeploy<OriginalResult>) -> Self {
        typed.sc_deploy_step
    }
}

impl<OriginalResult> From<ScDeployStep> for TypedScDeploy<OriginalResult> {
    fn from(untyped: ScDeployStep) -> Self {
        Self {
            sc_deploy_step: untyped,
            _phantom: PhantomData,
        }
    }
}

/// Helps with syntax. Allows the `TypedScDeploy` to call the `execute` operation directly.
///
/// The trait defines the connection to the executor.
pub trait TypedScDeployExecutor {
    fn execute_typed_sc_deploy<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScDeploy<OriginalResult>,
    ) -> (Address, RequestedResult)
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>;
}

impl<OriginalResult> TypedScDeploy<OriginalResult>
where
    OriginalResult: TopEncodeMulti,
{
    /// Executes the operation, on the given executor.
    pub fn execute<E: TypedScDeployExecutor, RequestedResult>(
        self,
        executor: &mut E,
    ) -> (Address, RequestedResult)
    where
        RequestedResult: CodecFrom<OriginalResult>,
    {
        executor.execute_typed_sc_deploy(self)
    }
}