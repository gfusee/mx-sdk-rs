use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/linked-list-repeat");

    blockchain.register_contract(
        "file:output/linked-list-repeat.wasm",
        linked_list_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn linked_list_repeat_rs() -> anyhow::Result<()> {
    world().run("scenarios/linked_list_repeat.scen.json")?;

    Ok(())
}

#[test]
fn linked_list_repeat_struct_rs() -> anyhow::Result<()> {
    world().run("scenarios/linked_list_repeat_struct.scen.json")?;

    Ok(())
}
