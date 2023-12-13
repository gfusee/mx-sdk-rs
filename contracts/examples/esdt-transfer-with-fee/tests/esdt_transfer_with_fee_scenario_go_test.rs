use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn claim_go() -> anyhow::Result<()> {
    world().run("scenarios/claim.scen.json")?;

    Ok(())
}

#[test]
fn deploy_go() -> anyhow::Result<()> {
    world().run("scenarios/deploy.scen.json")?;

    Ok(())
}

#[test]
fn setup_fees_and_transfer_go() -> anyhow::Result<()> {
    world().run("scenarios/setup_fees_and_transfer.scen.json")?;

    Ok(())
}
