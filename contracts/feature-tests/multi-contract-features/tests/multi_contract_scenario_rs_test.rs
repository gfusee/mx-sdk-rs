use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/multi-contract-features");

    blockchain.register_partial_contract::<multi_contract_features::AbiProvider, _>(
        "file:output/multi-contract-features.wasm",
        multi_contract_features::ContractBuilder,
        "multi-contract-features",
    );
    blockchain.register_partial_contract::<multi_contract_features::AbiProvider, _>(
        "file:output/multi-contract-features-view.wasm",
        multi_contract_features::ContractBuilder,
        "multi-contract-features-view",
    );
    blockchain.register_partial_contract::<multi_contract_features::AbiProvider, _>(
        "file:output/multi-contract-alt-impl.wasm",
        multi_contract_features::ContractBuilder,
        "multi-contract-alt-impl",
    );

    blockchain
}

#[test]
#[ignore = "not yet supported"]
fn mcf_alt_init_rs() -> anyhow::Result<()> {
    world().run("scenarios/mcf-alt-init.scen.json")?;

    Ok(())
}

#[test]
#[ignore = "not supported in principle"]
fn mcf_example_feature_rs() -> anyhow::Result<()> {
    world().run("scenarios/mcf-example-feature.scen.json")?;

    Ok(())
}

#[test]
fn mcf_external_get_rs() -> anyhow::Result<()> {
    world().run("scenarios/mcf-external-get.scen.json")?;

    Ok(())
}

#[test]
fn mcf_external_pure_rs() -> anyhow::Result<()> {
    world().run("scenarios/mcf-external-pure.scen.json")?;

    Ok(())
}
