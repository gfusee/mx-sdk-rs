use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn mcf_alt_init_go() -> anyhow::Result<()> {
    world().run("scenarios/mcf-alt-init.scen.json")?;

    Ok(())
}

#[test]
fn mcf_example_feature_go() -> anyhow::Result<()> {
    world().run("scenarios/mcf-example-feature.scen.json")?;

    Ok(())
}

#[test]
fn mcf_external_get_go() -> anyhow::Result<()> {
    world().run("scenarios/mcf-external-get.scen.json")?;

    Ok(())
}

#[test]
fn mcf_external_pure_go() -> anyhow::Result<()> {
    world().run("scenarios/mcf-external-pure.scen.json")?;

    Ok(())
}
