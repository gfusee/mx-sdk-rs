use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScenarioWorldRunnerErrors {
    #[error("ScenarioWorld's runner is poisoned")]
    Poisoned
}