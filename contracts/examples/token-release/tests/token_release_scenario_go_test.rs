use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn test_add_group_go() -> anyhow::Result<()> {
    world().run("scenarios/test-add-group.scen.json")?;

    Ok(())
}

#[test]
fn test_add_user_go() -> anyhow::Result<()> {
    world().run("scenarios/test-add-user.scen.json")?;

    Ok(())
}

#[test]
fn test_change_user_go() -> anyhow::Result<()> {
    world().run("scenarios/test-change-user.scen.json")?;

    Ok(())
}

#[test]
fn test_claim_go() -> anyhow::Result<()> {
    world().run("scenarios/test-claim.scen.json")?;

    Ok(())
}

#[test]
fn test_end_setup_go() -> anyhow::Result<()> {
    world().run("scenarios/test-end-setup.scen.json")?;

    Ok(())
}

#[test]
fn test_init_go() -> anyhow::Result<()> {
    world().run("scenarios/test-init.scen.json")?;

    Ok(())
}
