use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn deploy_erc_20_and_crowdfunding_go() -> anyhow::Result<()> {
    world().run("scenarios/deploy_erc20_and_crowdfunding.scen.json")?;

    Ok(())
}

#[test]
fn fund_with_insufficient_allowance_go() -> anyhow::Result<()> {
    world().run("scenarios/fund_with_insufficient_allowance.scen.json")?;

    Ok(())
}

#[test]
fn fund_with_sufficient_allowance_go() -> anyhow::Result<()> {
    world().run("scenarios/fund_with_sufficient_allowance.scen.json")?;

    Ok(())
}

#[test]
fn fund_without_allowance_go() -> anyhow::Result<()> {
    world().run("scenarios/fund_without_allowance.scen.json")?;

    Ok(())
}
