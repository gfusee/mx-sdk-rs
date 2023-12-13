use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn init_go() -> anyhow::Result<()> {
    world().run("scenarios/init.scen.json")?;

    Ok(())
}

#[test]
fn mint_nft_go() -> anyhow::Result<()> {
    world().run("scenarios/mint_nft.scen.json")?;

    Ok(())
}

#[test]
fn test_subscription_go() -> anyhow::Result<()> {
    world().run("scenarios/test_subscription.scen.json")?;

    Ok(())
}
