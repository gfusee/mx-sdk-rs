use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn factorial_go() -> anyhow::Result<()> {
    world().run("scenarios/factorial.scen.json")?;

    Ok(())
}
