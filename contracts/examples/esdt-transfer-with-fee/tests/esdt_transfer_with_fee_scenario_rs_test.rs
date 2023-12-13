use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/esdt-transfer-with-fee");

    blockchain.register_contract(
        "file:output/esdt-transfer-with-fee.wasm",
        esdt_transfer_with_fee::ContractBuilder,
    );
    blockchain
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
fn setup_fees_and_transfer_rs() -> anyhow::Result<()> {
    world().run("scenarios/setup_fees_and_transfer.scen.json")?;

    Ok(())
}
