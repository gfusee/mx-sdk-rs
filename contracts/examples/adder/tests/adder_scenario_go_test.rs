use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn adder_go() -> anyhow::Result<()> {
    world().run("scenarios/adder.scen.json")?;

    Ok(())
}

#[test]
fn interactor_trace_go() -> anyhow::Result<()> {
    world().run("scenarios/interactor_trace.scen.json")?;

    Ok(())
}
