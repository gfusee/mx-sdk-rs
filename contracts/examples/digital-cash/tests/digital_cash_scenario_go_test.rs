use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn claim_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/claim-egld.scen.json")?;

    Ok(())
}

#[test]
fn claim_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/claim-esdt.scen.json")?;

    Ok(())
}

#[test]
fn claim_fees_go() -> anyhow::Result<()> {
    world().run("scenarios/claim-fees.scen.json")?;

    Ok(())
}

#[test]
fn claim_multi_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/claim-multi-esdt.scen.json")?;

    Ok(())
}

#[test]
fn forward_go() -> anyhow::Result<()> {
    world().run("scenarios/forward.scen.json")?;

    Ok(())
}

#[test]
fn fund_egld_and_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/fund-egld-and-esdt.scen.json")?;

    Ok(())
}

#[test]
fn set_accounts_go() -> anyhow::Result<()> {
    world().run("scenarios/set-accounts.scen.json")?;

    Ok(())
}

#[test]
fn whitelist_blacklist_fee_token_go() -> anyhow::Result<()> {
    world().run("scenarios/whitelist-blacklist-fee-tokens.scen.json")?;

    Ok(())
}

#[test]
fn pay_fee_and_fund_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/pay-fee-and-fund-esdt.scen.json")?;

    Ok(())
}

#[test]
fn pay_fee_and_fund_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/pay-fee-and-fund-egld.scen.json")?;

    Ok(())
}

#[test]
fn withdraw_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/withdraw-egld.scen.json")?;

    Ok(())
}

#[test]
fn withdraw_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/withdraw-esdt.scen.json")?;

    Ok(())
}

#[test]
fn withdraw_multi_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/withdraw-multi-esdt.scen.json")?;

    Ok(())
}
