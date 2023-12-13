use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "file:../kitty-genetic-alg/output/kitty-genetic-alg.wasm",
        kitty_genetic_alg::ContractBuilder,
    );
    blockchain.register_contract(
        "file:output/kitty-ownership.wasm",
        kitty_ownership::ContractBuilder,
    );

    blockchain
}

#[test]
fn approve_siring_rs() -> anyhow::Result<()> {
    world().run("scenarios/approve_siring.scen.json")?;

    Ok(())
}

#[test]
fn breed_ok_rs() -> anyhow::Result<()> {
    world().run("scenarios/breed_ok.scen.json")?;

    Ok(())
}

#[test]
fn give_birth_rs() -> anyhow::Result<()> {
    world().run("scenarios/give_birth.scen.json")?;

    Ok(())
}

#[test]
fn init_rs() -> anyhow::Result<()> {
    world().run("scenarios/init.scen.json")?;

    Ok(())
}

#[test]
fn query_rs() -> anyhow::Result<()> {
    world().run("scenarios/query.scen.json")?;

    Ok(())
}

#[test]
fn setup_accounts_rs() -> anyhow::Result<()> {
    world().run("scenarios/setup_accounts.scen.json")?;

    Ok(())
}
