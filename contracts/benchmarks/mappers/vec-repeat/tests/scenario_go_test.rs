use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn vec_repeat_go() -> anyhow::Result<()> {
    world().run("scenarios/vec_repeat.scen.json")?;

    Ok(())
}

#[test]
fn vec_repeat_struct_go() -> anyhow::Result<()> {
    world().run("scenarios/vec_repeat_struct.scen.json")?;

    Ok(())
}
