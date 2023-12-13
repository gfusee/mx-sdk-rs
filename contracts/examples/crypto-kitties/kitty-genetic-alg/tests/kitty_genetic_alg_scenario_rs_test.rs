use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "file:output/kitty-genetic-alg.wasm",
        kitty_genetic_alg::ContractBuilder,
    );
    blockchain
}

#[test]
fn generate_kitty_genes_rs() -> anyhow::Result<()> {
    world().run("scenarios/generate-kitty-genes.scen.json")?;

    Ok(())
}

#[test]
fn init_rs() -> anyhow::Result<()> {
    world().run("scenarios/init.scen.json")?;

    Ok(())
}
