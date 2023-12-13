use std::path::{Path, PathBuf};

use crate::{scenario::ScenarioRunner, scenario_model::*};

use super::ScenarioTrace;

pub struct ScenarioTraceFile {
    full_path: PathBuf,
    // TODO: some caching/flushing might be a good idea, at least for some situations
    // this involves adding some cache/flush methods to ScenarioRunner
}

impl ScenarioTraceFile {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        ScenarioTraceFile {
            full_path: path.as_ref().into(),
        }
    }

    fn with_tracer(&self, f: impl FnOnce(&mut ScenarioTrace) -> anyhow::Result<()>) -> anyhow::Result<()> {
        let mut tracer = ScenarioTrace::default();
        if self.full_path.is_file() {
            tracer.load_scenario_trace(&self.full_path);
        }

        f(&mut tracer)?;
        tracer.write_scenario_trace(&self.full_path);

        Ok(())
    }
}

impl ScenarioRunner for ScenarioTraceFile {
    fn run_external_steps(&mut self, step: &ExternalStepsStep) -> anyhow::Result<()> {
        self.with_tracer(|tracer| tracer.run_external_steps(step))
    }

    fn run_set_state_step(&mut self, step: &SetStateStep) -> anyhow::Result<()> {
        self.with_tracer(|tracer| tracer.run_set_state_step(step))
    }

    fn run_sc_call_step(&mut self, step: &mut ScCallStep) -> anyhow::Result<()> {
        self.with_tracer(|tracer| tracer.run_sc_call_step(step))
    }

    fn run_multi_sc_call_step(&mut self, steps: &mut [ScCallStep]) -> anyhow::Result<()> {
        self.with_tracer(|tracer| tracer.run_multi_sc_call_step(steps))
    }

    fn run_multi_sc_deploy_step(&mut self, steps: &mut [ScDeployStep]) -> anyhow::Result<()> {
        self.with_tracer(|tracer| tracer.run_multi_sc_deploy_step(steps))
    }

    fn run_sc_query_step(&mut self, step: &mut ScQueryStep) -> anyhow::Result<()> {
        self.with_tracer(|tracer| tracer.run_sc_query_step(step))
    }

    fn run_sc_deploy_step(&mut self, step: &mut ScDeployStep) -> anyhow::Result<()> {
        self.with_tracer(|tracer| tracer.run_sc_deploy_step(step))
    }

    fn run_transfer_step(&mut self, step: &TransferStep) -> anyhow::Result<()> {
        self.with_tracer(|tracer| tracer.run_transfer_step(step))
    }

    fn run_validator_reward_step(&mut self, step: &ValidatorRewardStep) -> anyhow::Result<()> {
        self.with_tracer(|tracer| tracer.run_validator_reward_step(step))
    }

    fn run_check_state_step(&mut self, step: &CheckStateStep) -> anyhow::Result<()> {
        self.with_tracer(|tracer| tracer.run_check_state_step(step))
    }

    fn run_dump_state_step(&mut self) -> anyhow::Result<()> {
        self.with_tracer(|tracer| tracer.run_dump_state_step())
    }
}
