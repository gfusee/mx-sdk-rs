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
#[ignore]
fn reject_transfer_go() -> anyhow::Result<()> {
    world().run("scenarios/reject_transfer.scen.json")?;

    Ok(())
}

#[test]
fn simple_transfer_full_go() -> anyhow::Result<()> {
    world().run("scenarios/simple_transfer_full.scen.json")?;

    Ok(())
}

#[test]
fn simple_transfer_full_wrong_token_go() -> anyhow::Result<()> {
    world().run("scenarios/simple_transfer_full_wrong_token.scen.json")?;

    Ok(())
}

#[test]
fn simple_transfer_half_go() -> anyhow::Result<()> {
    world().run("scenarios/simple_transfer_half.scen.json")?;

    Ok(())
}
