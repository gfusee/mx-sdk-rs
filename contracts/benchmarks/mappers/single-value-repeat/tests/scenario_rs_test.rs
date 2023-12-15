use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/single-value-repeat");

    blockchain.register_contract(
        "file:output/single-value-repeat.wasm",
        single_value_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn single_value_repeat_rs() -> anyhow::Result<()> {
    world().run("scenarios/single_value_repeat.scen.json")?;

    Ok(())
}

#[test]
fn single_value_repeat_struct_rs() -> anyhow::Result<()> {
    world().run("scenarios/single_value_repeat_struct.scen.json")?;

    Ok(())
}
