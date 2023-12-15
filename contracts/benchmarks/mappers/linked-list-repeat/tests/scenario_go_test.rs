use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn linked_list_repeat_go() -> anyhow::Result<()> {
    world().run("scenarios/linked_list_repeat.scen.json")?;

    Ok(())
}

#[test]
fn linked_list_repeat_struct_go() -> anyhow::Result<()> {
    world().run("scenarios/linked_list_repeat_struct.scen.json")?;

    Ok(())
}
