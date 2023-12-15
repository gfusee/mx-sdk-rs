use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn map_repeat_go() -> anyhow::Result<()> {
    world().run("scenarios/map_repeat.scen.json")?;

    Ok(())
}

#[test]
fn map_repeat_struct_go() -> anyhow::Result<()> {
    world().run("scenarios/map_repeat_struct.scen.json")?;

    Ok(())
}
