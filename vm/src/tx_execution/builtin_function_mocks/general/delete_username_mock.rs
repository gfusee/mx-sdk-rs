use crate::tx_execution::{builtin_function_names::DELETE_USERNAME_FUNC_NAME, BlockchainVMRef};

use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct DeleteUsername;

impl BuiltinFunction for DeleteUsername {
    fn name(&self) -> &str {
        DELETE_USERNAME_FUNC_NAME
    }

    fn execute<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        _vm: &BlockchainVMRef,
        _f: F,
    ) -> anyhow::Result<(TxResult, BlockchainUpdate)>
    where
        F: FnOnce() -> anyhow::Result<()>,
    {
        if !tx_input.args.is_empty() {
            return Ok(
                (
                    TxResult::from_vm_error("DeleteUserName expects no arguments"),
                    BlockchainUpdate::empty(),
                )
            );
        }

        tx_cache.with_account_mut(&tx_input.to, |account| {
            account.username = Vec::new();
        })?;

        Ok((TxResult::empty(), tx_cache.into_blockchain_updates()))
    }
}
