use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn buy_go() -> anyhow::Result<()> {
    world().run("scenarios/buy.scen.json")?;

    Ok(())
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
fn deposit_go() -> anyhow::Result<()> {
    world().run("scenarios/deposit.scen.json")?;

    Ok(())
}

#[test]
fn deposit_more_view_go() -> anyhow::Result<()> {
    world().run("scenarios/deposit_more_view.scen.json")?;

    Ok(())
}

#[test]
fn sell_go() -> anyhow::Result<()> {
    world().run("scenarios/sell.scen.json")?;

    Ok(())
}

#[test]
fn set_bonding_curve_go() -> anyhow::Result<()> {
    world().run("scenarios/set_bonding_curve.scen.json")?;

    Ok(())
}
