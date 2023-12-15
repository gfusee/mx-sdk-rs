use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn managed_error_message_go() -> anyhow::Result<()> {
    world().run("scenarios/managed_error_message.scen.json")?;

    Ok(())
}

#[test]
fn sc_format_go() -> anyhow::Result<()> {
    world().run("scenarios/sc_format.scen.json")?;

    Ok(())
}
