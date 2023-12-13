use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn buy_nft_go() -> anyhow::Result<()> {
    world().run("scenarios/buy_nft.scen.json")?;

    Ok(())
}

#[test]
fn create_nft_go() -> anyhow::Result<()> {
    world().run("scenarios/create_nft.scen.json")?;

    Ok(())
}

#[test]
fn init_go() -> anyhow::Result<()> {
    world().run("scenarios/init.scen.json")?;

    Ok(())
}
