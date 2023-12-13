use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn generate_kitty_genes_go() -> anyhow::Result<()> {
    world().run("scenarios/generate-kitty-genes.scen.json")?;

    Ok(())
}

#[test]
fn init_go() -> anyhow::Result<()> {
    world().run("scenarios/init.scen.json")?;

    Ok(())
}
