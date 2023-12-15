use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn use_module_claim_developer_rewards_go() -> anyhow::Result<()> {
    world().run("scenarios/use_module_claim_developer_rewards.scen.json")?;

    Ok(())
}

#[test]
#[ignore = "uses multi-level async"]
fn use_module_dns_register_go() -> anyhow::Result<()> {
    world().run("scenarios/use_module_dns_register.scen.json")?;

    Ok(())
}

#[test]
fn use_module_features_go() -> anyhow::Result<()> {
    world().run("scenarios/use_module_features.scen.json")?;

    Ok(())
}

#[test]
fn use_module_internal_go() -> anyhow::Result<()> {
    world().run("scenarios/use_module_internal.scen.json")?;

    Ok(())
}

#[test]
fn use_module_no_endpoint_go() -> anyhow::Result<()> {
    world().run("scenarios/use_module_no_endpoint.scen.json")?;

    Ok(())
}

#[test]
fn use_module_ongoing_operation_example_go() -> anyhow::Result<()> {
    world().run("scenarios/use_module_ongoing_operation_example.scen.json")?;

    Ok(())
}

#[test]
fn use_module_only_admin_go() -> anyhow::Result<()> {
    world().run("scenarios/use_module_only_admin.scen.json")?;

    Ok(())
}

#[test]
fn use_module_only_owner_go() -> anyhow::Result<()> {
    world().run("scenarios/use_module_only_owner.scen.json")?;

    Ok(())
}

#[test]
fn use_module_pause_go() -> anyhow::Result<()> {
    world().run("scenarios/use_module_pause.scen.json")?;

    Ok(())
}
