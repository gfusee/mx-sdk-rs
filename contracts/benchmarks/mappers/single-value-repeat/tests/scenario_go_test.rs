use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn single_value_repeat_go() -> anyhow::Result<()> {
    world().run("scenarios/single_value_repeat.scen.json")?;

    Ok(())
}

#[test]
fn single_value_repeat_struct_go() -> anyhow::Result<()> {
    world().run("scenarios/single_value_repeat_struct.scen.json")?;

    Ok(())
}
