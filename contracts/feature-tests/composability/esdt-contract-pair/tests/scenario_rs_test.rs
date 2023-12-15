use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "file:first-contract/output/first-contract.wasm",
        first_contract::ContractBuilder,
    );

    blockchain.register_contract(
        "file:second-contract/output/second-contract.wasm",
        second_contract::ContractBuilder,
    );
    blockchain
}

#[test]
fn init_rs() -> anyhow::Result<()> {
    world().run("scenarios/init.scen.json")?;

    Ok(())
}

// TODO: implement ESDTTransfer + async call
#[test]
#[ignore]
fn reject_transfer_rs() -> anyhow::Result<()> {
    world().run("scenarios/reject_transfer.scen.json")?;

    Ok(())
}

#[test]
fn simple_transfer_full_rs() -> anyhow::Result<()> {
    world().run("scenarios/simple_transfer_full.scen.json")?;

    Ok(())
}

#[test]
fn simple_transfer_full_wrong_token_rs() -> anyhow::Result<()> {
    world().run("scenarios/simple_transfer_full_wrong_token.scen.json")?;

    Ok(())
}

#[test]
fn simple_transfer_half_rs() -> anyhow::Result<()> {
    world().run("scenarios/simple_transfer_half.scen.json")?;

    Ok(())
}
