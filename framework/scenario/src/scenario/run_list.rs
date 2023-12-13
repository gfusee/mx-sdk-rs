use crate::scenario_model::*;

use super::ScenarioRunner;

/// Aggregates several scenario runners into one, and calls them in order.
///
/// The empty object can act as a placeholder, in case we want to provide a `ScenarioRunner` that does nothing.
#[derive(Default)]
pub struct ScenarioRunnerList {
    list: Vec<Box<dyn ScenarioRunner>>,
}

impl ScenarioRunnerList {
    pub const fn empty() -> Self {
        ScenarioRunnerList { list: Vec::new() }
    }

    pub fn push<R>(&mut self, runner: R)
    where
        R: ScenarioRunner + 'static,
    {
        self.list.push(Box::new(runner));
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

impl ScenarioRunner for ScenarioRunnerList {
    fn run_external_steps(&mut self, step: &ExternalStepsStep) -> anyhow::Result<()> {
        for runner in self.list.iter_mut() {
            runner.run_external_steps(step)?;
        }

        Ok(())
    }

    fn run_set_state_step(&mut self, step: &SetStateStep) -> anyhow::Result<()> {
        for runner in self.list.iter_mut() {
            runner.run_set_state_step(step)?;
        }

        Ok(())
    }

    fn run_sc_call_step(&mut self, step: &mut ScCallStep) -> anyhow::Result<()> {
        for runner in self.list.iter_mut() {
            runner.run_sc_call_step(step)?;
        }

        Ok(())
    }

    fn run_multi_sc_call_step(&mut self, steps: &mut [ScCallStep]) -> anyhow::Result<()> {
        for runner in self.list.iter_mut() {
            for step in steps.iter_mut() {
                runner.run_sc_call_step(step)?;
            }
        }

        Ok(())
    }

    fn run_multi_sc_deploy_step(&mut self, steps: &mut [ScDeployStep]) -> anyhow::Result<()> {
        for runner in self.list.iter_mut() {
            for step in steps.iter_mut() {
                runner.run_sc_deploy_step(step)?;
            }
        }

        Ok(())
    }

    fn run_sc_query_step(&mut self, step: &mut ScQueryStep) -> anyhow::Result<()> {
        for runner in self.list.iter_mut() {
            runner.run_sc_query_step(step)?;
        }

        Ok(())
    }

    fn run_sc_deploy_step(&mut self, step: &mut ScDeployStep) -> anyhow::Result<()> {
        for runner in self.list.iter_mut() {
            runner.run_sc_deploy_step(step)?;
        }

        Ok(())
    }

    fn run_transfer_step(&mut self, step: &TransferStep) -> anyhow::Result<()> {
        for runner in self.list.iter_mut() {
            runner.run_transfer_step(step)?;
        }

        Ok(())
    }

    fn run_validator_reward_step(&mut self, step: &ValidatorRewardStep) -> anyhow::Result<()> {
        for runner in self.list.iter_mut() {
            runner.run_validator_reward_step(step)?;
        }

        Ok(())
    }

    fn run_check_state_step(&mut self, step: &CheckStateStep) -> anyhow::Result<()> {
        for runner in self.list.iter_mut() {
            runner.run_check_state_step(step)?;
        }

        Ok(())
    }

    fn run_dump_state_step(&mut self) -> anyhow::Result<()> {
        for runner in self.list.iter_mut() {
            runner.run_dump_state_step()?;
        }

        Ok(())
    }
}
