use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain
        .set_current_dir_from_workspace("contracts/feature-tests/rust-testing-framework-tester");

    blockchain.register_contract(
        "file:output/rust-testing-framework-tester.wasm",
        rust_testing_framework_tester::ContractBuilder,
    );
    blockchain
}

#[test]
fn test_rs() -> anyhow::Result<()> {
    world().run("scenarios/test.scen.json")?;

    Ok(())
}

#[test]
fn test_esdt_generation_rs() -> anyhow::Result<()> {
    world().run("scenarios/test_esdt_generation.scen.json")?;

    Ok(())
}

#[test]
fn test_multiple_sc_rs() -> anyhow::Result<()> {
    world().run("scenarios/test_multiple_sc.scen.json")?;

    Ok(())
}

#[test]
#[ignore = "not supported"]
fn trace_deploy_rs() -> anyhow::Result<()> {
    world().run("scenarios/trace-deploy.scen.json")?;

    Ok(())
}
