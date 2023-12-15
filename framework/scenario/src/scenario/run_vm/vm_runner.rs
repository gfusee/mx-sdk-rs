use anyhow::bail;
use crate::{
    debug_executor::ContractMapRef,
    multiversx_chain_vm::BlockchainMock,
    scenario::{model::*, ScenarioRunner},
};
use crate::scenario::run_vm::scenario_world_errors::ScenarioWorldRunnerErrors;

/// Wraps calls to the blockchain mock,
/// while implementing the StepRunner interface.
#[derive(Default, Debug)]
pub struct ScenarioVMRunner {
    pub contract_map_ref: ContractMapRef,
    pub blockchain_mock: BlockchainMock,
    poisoned: bool
}

impl ScenarioVMRunner {
    pub fn new() -> Self {
        let contract_map_ref = ContractMapRef::new();
        let blockchain_mock = BlockchainMock::new(Box::new(contract_map_ref.clone()));
        ScenarioVMRunner {
            contract_map_ref,
            blockchain_mock,
            poisoned: false,
        }
    }

    fn with_failable_run<F>(
        &mut self,
        f: F
    ) -> anyhow::Result<()>
        where
            F: FnOnce(&mut Self) -> anyhow::Result<()>
    {
        if self.poisoned {
            bail!(ScenarioWorldRunnerErrors::Poisoned)
        }

        let run_result = f(self);

        if run_result.is_err() {
            self.poisoned = true;
        }

        run_result
    }
}

impl ScenarioRunner for ScenarioVMRunner {
    fn run_external_steps(&mut self, _step: &ExternalStepsStep) -> anyhow::Result<()> {
        bail!("cannot call directly as such")
    }

    fn run_set_state_step(&mut self, step: &SetStateStep) -> anyhow::Result<()> {
        self.with_failable_run(|self_ref| {
            Ok(self_ref.perform_set_state(step))
        })
    }

    fn run_sc_call_step(&mut self, step: &mut ScCallStep) -> anyhow::Result<()> {
        self.with_failable_run(|self_ref| {
            self_ref.perform_sc_call_update_results(step)
        })
    }

    fn run_multi_sc_call_step(&mut self, steps: &mut [ScCallStep]) -> anyhow::Result<()> {
        self.with_failable_run(|self_ref| {
            for step in steps {
                self_ref.perform_sc_call_update_results(step)?;
            }

            Ok(())
        })
    }

    fn run_multi_sc_deploy_step(&mut self, steps: &mut [ScDeployStep]) -> anyhow::Result<()> {
        self.with_failable_run(|self_ref| {
            for step in steps.iter_mut() {
                self_ref.perform_sc_deploy_update_results(step)?;
            }

            Ok(())
        })
    }

    fn run_sc_query_step(&mut self, step: &mut ScQueryStep) -> anyhow::Result<()> {
        self.with_failable_run(|self_ref| {
            self_ref.perform_sc_query_update_results(step)
        })
    }

    fn run_sc_deploy_step(&mut self, step: &mut ScDeployStep) -> anyhow::Result<()> {
        self.with_failable_run(|self_ref| {
            self_ref.perform_sc_deploy_update_results(step)
        })
    }

    fn run_transfer_step(&mut self, step: &TransferStep) -> anyhow::Result<()> {
        self.with_failable_run(|self_ref| {
            self_ref.perform_transfer(step)
        })
    }

    fn run_validator_reward_step(&mut self, step: &ValidatorRewardStep) -> anyhow::Result<()> {
        self.with_failable_run(|self_ref| {
            Ok(self_ref.perform_validator_reward(step))
        })
    }

    fn run_check_state_step(&mut self, step: &CheckStateStep) -> anyhow::Result<()> {
        self.with_failable_run(|self_ref| {
            Ok(self_ref.perform_check_state(step))
        })
    }

    fn run_dump_state_step(&mut self) -> anyhow::Result<()> {
        self.with_failable_run(|self_ref| {
            Ok(self_ref.perform_dump_state())
        })
    }
}
