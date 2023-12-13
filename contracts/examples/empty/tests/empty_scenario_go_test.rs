use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn empty_go() -> anyhow::Result<()> {
    world().run("scenarios/empty.scen.json")?;

    Ok(())
}
