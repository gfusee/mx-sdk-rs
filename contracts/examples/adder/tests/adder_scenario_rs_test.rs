use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract("file:output/adder.wasm", adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_rs() -> anyhow::Result<()> {
    world().run("scenarios/adder.scen.json")?;

    Ok(())
}

#[test]
fn interactor_trace_rs() -> anyhow::Result<()> {
    world().run("scenarios/interactor_trace.scen.json")?;

    Ok(())
}
