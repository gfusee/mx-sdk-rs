use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
#[ignore]
fn mmap_get_go() -> anyhow::Result<()> {
    world().run("scenarios/mmap_get.scen.json")?;

    Ok(())
}

#[test]
#[ignore]
fn mmap_remove_go() -> anyhow::Result<()> {
    world().run("scenarios/mmap_remove.scen.json")?;

    Ok(())
}
