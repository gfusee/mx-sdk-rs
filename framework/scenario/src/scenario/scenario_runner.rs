use super::model::*;

/// Allows caller to process a single scenario step, no matter what this means concretely.
///
/// Abstracts away implementation, can be
/// - a simulation using any executor,
/// - calls to a blockchain,
/// - collecting/exporting the scenario,
/// - something else.
pub trait ScenarioRunner: Send + Sync {
    fn run_external_steps(&mut self, step: &ExternalStepsStep)-> anyhow::Result<()>;

    fn run_set_state_step(&mut self, step: &SetStateStep) -> anyhow::Result<()>;

    fn run_sc_call_step(&mut self, step: &mut ScCallStep) -> anyhow::Result<()>;

    fn run_multi_sc_call_step(&mut self, steps: &mut [ScCallStep]) -> anyhow::Result<()>;

    fn run_multi_sc_deploy_step(&mut self, steps: &mut [ScDeployStep]) -> anyhow::Result<()>;

    fn run_sc_query_step(&mut self, step: &mut ScQueryStep) -> anyhow::Result<()>;

    fn run_sc_deploy_step(&mut self, step: &mut ScDeployStep) -> anyhow::Result<()>;

    fn run_transfer_step(&mut self, step: &TransferStep) -> anyhow::Result<()>;

    fn run_validator_reward_step(&mut self, step: &ValidatorRewardStep) -> anyhow::Result<()>;

    fn run_check_state_step(&mut self, step: &CheckStateStep) -> anyhow::Result<()>;

    fn run_dump_state_step(&mut self) -> anyhow::Result<()>;

    /// Utility method for running all steps in a scenario.
    fn run_scenario(&mut self, scenario: &Scenario) -> anyhow::Result<()> {
        let mut steps = scenario.steps.clone();
        for step in steps.iter_mut() {
            match step {
                Step::ExternalSteps(external_steps_step) => {
                    self.run_external_steps(external_steps_step)?;
                },
                Step::SetState(set_state_step) => {
                    self.run_set_state_step(set_state_step)?;
                },
                Step::ScCall(sc_call_step) => {
                    self.run_sc_call_step(sc_call_step)?;
                },
                Step::ScQuery(sc_query_step) => {
                    self.run_sc_query_step(sc_query_step)?;
                },
                Step::ScDeploy(sc_deploy_step) => {
                    self.run_sc_deploy_step(sc_deploy_step)?;
                },
                Step::Transfer(transfer_step) => {
                    self.run_transfer_step(transfer_step)?;
                },
                Step::ValidatorReward(validator_reward_step) => {
                    self.run_validator_reward_step(validator_reward_step)?;
                },
                Step::CheckState(check_state_step) => {
                    self.run_check_state_step(check_state_step)?;
                },
                Step::DumpState(_) => {
                    self.run_dump_state_step()?;
                },
            }
        }

        Ok(())
    }
}
