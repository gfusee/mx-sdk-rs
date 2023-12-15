use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/set-repeat");

    blockchain.register_contract("file:output/set-repeat.wasm", set_repeat::ContractBuilder);
    blockchain
}

#[test]
fn set_repeat_rs() -> anyhow::Result<()> {
    world().run("scenarios/set_repeat.scen.json")?;

    Ok(())
}

#[test]
fn set_repeat_struct_rs() -> anyhow::Result<()> {
    world().run("scenarios/set_repeat_struct.scen.json")?;

    Ok(())
}
