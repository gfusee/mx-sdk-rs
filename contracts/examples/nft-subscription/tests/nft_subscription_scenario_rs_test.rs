use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/nft-subscription");

    blockchain.register_contract(
        "file:output/nft-subscription.wasm",
        nft_subscription::ContractBuilder,
    );
    blockchain
}

#[test]
fn init_rs() -> anyhow::Result<()> {
    world().run("scenarios/init.scen.json")?;

    Ok(())
}

#[test]
fn mint_nft_rs() -> anyhow::Result<()> {
    world().run("scenarios/mint_nft.scen.json")?;

    Ok(())
}

#[test]
fn test_subscription_rs() -> anyhow::Result<()> {
    world().run("scenarios/test_subscription.scen.json")?;

    Ok(())
}
