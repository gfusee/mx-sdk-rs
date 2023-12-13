use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn init_go() -> anyhow::Result<()> {
    world().run("scenarios/init.scen.json")?;

    Ok(())
}

#[test]
fn pause_and_unpause_go() -> anyhow::Result<()> {
    world().run("scenarios/pause-and-unpause.scen.json")?;

    Ok(())
}
