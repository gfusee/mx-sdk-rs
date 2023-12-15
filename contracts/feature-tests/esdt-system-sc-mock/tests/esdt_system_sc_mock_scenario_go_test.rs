use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn esdt_system_sc_go() -> anyhow::Result<()> {
    world().run("scenarios/esdt_system_sc.scen.json")?;

    Ok(())
}
