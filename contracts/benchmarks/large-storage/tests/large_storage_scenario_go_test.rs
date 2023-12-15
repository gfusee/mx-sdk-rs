use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn large_storage_go() -> anyhow::Result<()> {
    world().run("scenarios/large_storage.scen.json")?;

    Ok(())
}
