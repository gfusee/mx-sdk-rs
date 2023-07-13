use multiversx_sc_scenario::{
    mandos_system::ScenarioRunner,
    scenario_model::{AddressValue, ScCallStep, ScDeployStep, TxResponse},
};
use multiversx_sdk::data::transaction::Transaction;

use crate::Interactor;

pub trait TransactionSpec {
    fn to_transaction(&self, interactor: &Interactor) -> Transaction;

    fn to_address(&self) -> &AddressValue;

    fn run_step(&self, step_runner: &mut dyn ScenarioRunner);

    fn set_response(&mut self, tx_response: TxResponse);
}

impl TransactionSpec for ScCallStep {
    fn to_transaction(&self, interactor: &Interactor) -> Transaction {
        interactor.tx_call_to_blockchain_tx(&self.tx)
    }

    fn to_address(&self) -> &AddressValue {
        &self.tx.from
    }

    fn run_step(&self, step_runner: &mut dyn ScenarioRunner) {
        step_runner.run_sc_call_step(self);
    }

    fn set_response(&mut self, tx_response: TxResponse) {
        self.response = Some(tx_response);
    }
}

impl TransactionSpec for ScDeployStep {
    fn to_transaction(&self, interactor: &Interactor) -> Transaction {
        interactor.sc_deploy_to_blockchain_tx(self)
    }

    fn to_address(&self) -> &AddressValue {
        &self.tx.from
    }

    fn run_step(&self, step_runner: &mut dyn ScenarioRunner) {
        step_runner.run_sc_deploy_step(self);
    }

    fn set_response(&mut self, tx_response: TxResponse) {
        self.response = Some(tx_response);
    }
}