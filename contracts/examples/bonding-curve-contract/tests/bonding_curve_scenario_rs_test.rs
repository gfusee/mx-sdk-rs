use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "file:output/bonding-curve-contract.wasm",
        bonding_curve_contract::ContractBuilder,
    );
    blockchain
}

#[test]
fn buy_rs() -> anyhow::Result<()> {
    world().run("scenarios/buy.scen.json")?;

    Ok(())
}

#[test]
fn claim_rs() -> anyhow::Result<()> {
    world().run("scenarios/claim.scen.json")?;

    Ok(())
}

#[test]
fn deploy_rs() -> anyhow::Result<()> {
    world().run("scenarios/deploy.scen.json")?;

    Ok(())
}

#[test]
fn deposit_rs() -> anyhow::Result<()> {
    world().run("scenarios/deposit.scen.json")?;

    Ok(())
}

#[test]
fn deposit_more_view_rs() -> anyhow::Result<()> {
    world().run("scenarios/deposit_more_view.scen.json")?;

    Ok(())
}

#[test]
fn sell_rs() -> anyhow::Result<()> {
    world().run("scenarios/sell.scen.json")?;

    Ok(())
}

#[test]
fn set_bonding_curve_rs() -> anyhow::Result<()> {
    world().run("scenarios/set_bonding_curve.scen.json")?;

    Ok(())
}
