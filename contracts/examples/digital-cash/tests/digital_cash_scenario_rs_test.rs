use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/digital-cash");

    blockchain.register_contract(
        "file:output/digital-cash.wasm",
        digital_cash::ContractBuilder,
    );
    blockchain
}

#[test]
fn claim_egld_rs() -> anyhow::Result<()> {
    world().run("scenarios/claim-egld.scen.json")?;

    Ok(())
}

#[test]
fn claim_esdt_rs() -> anyhow::Result<()> {
    world().run("scenarios/claim-esdt.scen.json")?;

    Ok(())
}

#[test]
fn claim_fees_rs() -> anyhow::Result<()> {
    world().run("scenarios/claim-fees.scen.json")?;

    Ok(())
}

#[test]
fn claim_multi_esdt_rs() -> anyhow::Result<()> {
    world().run("scenarios/claim-multi-esdt.scen.json")?;

    Ok(())
}

#[test]
fn forward_rs() -> anyhow::Result<()> {
    world().run("scenarios/forward.scen.json")?;

    Ok(())
}

#[test]
fn fund_egld_and_esdt_rs() -> anyhow::Result<()> {
    world().run("scenarios/fund-egld-and-esdt.scen.json")?;

    Ok(())
}

#[test]
fn set_accounts_rs() -> anyhow::Result<()> {
    world().run("scenarios/set-accounts.scen.json")?;

    Ok(())
}

#[test]
fn whitelist_blacklist_fee_token_rs() -> anyhow::Result<()> {
    world().run("scenarios/whitelist-blacklist-fee-tokens.scen.json")?;

    Ok(())
}

#[test]
fn pay_fee_and_fund_esdt_rs() -> anyhow::Result<()> {
    world().run("scenarios/pay-fee-and-fund-esdt.scen.json")?;

    Ok(())
}

#[test]
fn pay_fee_and_fund_egld_rs() -> anyhow::Result<()> {
    world().run("scenarios/pay-fee-and-fund-egld.scen.json")?;

    Ok(())
}

#[test]
fn withdraw_egld_rs() -> anyhow::Result<()> {
    world().run("scenarios/withdraw-egld.scen.json")?;

    Ok(())
}

#[test]
fn withdraw_esdt_rs() -> anyhow::Result<()> {
    world().run("scenarios/withdraw-esdt.scen.json")?;

    Ok(())
}

#[test]
fn withdraw_multi_esdt_rs() -> anyhow::Result<()> {
    world().run("scenarios/withdraw-multi-esdt.scen.json")?;

    Ok(())
}
