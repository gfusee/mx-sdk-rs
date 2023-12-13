use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn approve_siring_go() -> anyhow::Result<()> {
    world().run("scenarios/approve_siring.scen.json")?;

    Ok(())
}

#[test]
fn breed_ok_go() -> anyhow::Result<()> {
    world().run("scenarios/breed_ok.scen.json")?;

    Ok(())
}

#[test]
fn give_birth_go() -> anyhow::Result<()> {
    world().run("scenarios/give_birth.scen.json")?;

    Ok(())
}

#[test]
fn init_go() -> anyhow::Result<()> {
    world().run("scenarios/init.scen.json")?;

    Ok(())
}

#[test]
fn query_go() -> anyhow::Result<()> {
    world().run("scenarios/query.scen.json")?;

    Ok(())
}

#[test]
fn setup_accounts_go() -> anyhow::Result<()> {
    world().run("scenarios/setup_accounts.scen.json")?;

    Ok(())
}
