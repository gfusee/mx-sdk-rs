use anyhow::bail;
use crate::{
    facade::ScenarioWorld,
    scenario::{model::*, ScenarioRunner},
};

use super::scenario_world::Backend;

impl ScenarioWorld {
    pub fn for_each_runner_mut<F: FnMut(&mut dyn ScenarioRunner) -> anyhow::Result<()>>(&mut self, mut f: F) -> anyhow::Result<()> {
        match &mut self.backend {
            Backend::Debugger(cd_debugger) => {
                f(&mut cd_debugger.vm_runner)?;
                if let Some(trace) = &mut cd_debugger.trace {
                    f(trace)?;
                }

                Ok(())
            },
            Backend::VmGoBackend => {
                bail!("the VM Go backend does not support step-by-step execution")
            },
        }
    }
}

impl ScenarioRunner for ScenarioWorld {
    fn run_external_steps(&mut self, step: &ExternalStepsStep) -> anyhow::Result<()> {
        self.for_each_runner_mut(|runner| runner.run_external_steps(step))
    }

    fn run_set_state_step(&mut self, step: &SetStateStep) -> anyhow::Result<()> {
        self.for_each_runner_mut(|runner| runner.run_set_state_step(step))
    }

    fn run_sc_call_step(&mut self, step: &mut ScCallStep) -> anyhow::Result<()> {
        self.for_each_runner_mut(|runner| runner.run_sc_call_step(step))
    }

    fn run_multi_sc_call_step(&mut self, steps: &mut [ScCallStep]) -> anyhow::Result<()> {
        self.for_each_runner_mut(|runner| runner.run_multi_sc_call_step(steps))
    }

    fn run_multi_sc_deploy_step(&mut self, steps: &mut [ScDeployStep]) -> anyhow::Result<()> {
        self.for_each_runner_mut(|runner| runner.run_multi_sc_deploy_step(steps))
    }

    fn run_sc_query_step(&mut self, step: &mut ScQueryStep) -> anyhow::Result<()> {
        self.for_each_runner_mut(|runner| runner.run_sc_query_step(step))
    }

    fn run_sc_deploy_step(&mut self, step: &mut ScDeployStep) -> anyhow::Result<()> {
        self.for_each_runner_mut(|runner| runner.run_sc_deploy_step(step))
    }

    fn run_transfer_step(&mut self, step: &TransferStep) -> anyhow::Result<()> {
        self.for_each_runner_mut(|runner| runner.run_transfer_step(step))
    }

    fn run_validator_reward_step(&mut self, step: &ValidatorRewardStep) -> anyhow::Result<()> {
        self.for_each_runner_mut(|runner| runner.run_validator_reward_step(step))
    }

    fn run_check_state_step(&mut self, step: &CheckStateStep) -> anyhow::Result<()> {
        self.for_each_runner_mut(|runner| runner.run_check_state_step(step))
    }

    fn run_dump_state_step(&mut self) -> anyhow::Result<()> {
        self.for_each_runner_mut(|runner| runner.run_dump_state_step())
    }
}
