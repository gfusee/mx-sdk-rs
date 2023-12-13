use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/nft-minter");

    blockchain.register_contract("file:output/nft-minter.wasm", nft_minter::ContractBuilder);
    blockchain
}

#[test]
fn buy_nft_rs() -> anyhow::Result<()> {
    world().run("scenarios/buy_nft.scen.json")?;

    Ok(())
}

#[test]
fn create_nft_rs() -> anyhow::Result<()> {
    world().run("scenarios/create_nft.scen.json")?;

    Ok(())
}

#[test]
fn init_rs() -> anyhow::Result<()> {
    world().run("scenarios/init.scen.json")?;

    Ok(())
}
