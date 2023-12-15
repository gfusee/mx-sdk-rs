use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn test_go() -> anyhow::Result<()> {
    world().run("scenarios/test.scen.json")?;

    Ok(())
}

#[test]
fn test_esdt_generation_go() -> anyhow::Result<()> {
    world().run("scenarios/test_esdt_generation.scen.json")?;

    Ok(())
}

#[test]
fn test_multiple_sc_go() -> anyhow::Result<()> {
    world().run("scenarios/test_multiple_sc.scen.json")?;

    Ok(())
}

#[test]
#[ignore = "not supported"]
fn trace_deploy_go() -> anyhow::Result<()> {
    world().run("scenarios/trace-deploy.scen.json")?;

    Ok(())
}
