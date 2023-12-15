use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::new()
}

#[test]
fn local_path_test() -> anyhow::Result<()> {
    world().run("tests/scenarios-self/path_test.scen.json")?;

    Ok(())
}

#[test]
fn nested_path_test() -> anyhow::Result<()> {
    world().run("tests/scenarios-self/external_steps/external_path_test.scen.json")?;

    Ok(())
}
