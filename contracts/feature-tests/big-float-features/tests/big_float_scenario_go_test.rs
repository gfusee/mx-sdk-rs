use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn big_float_new_from_big_int_go() -> anyhow::Result<()> {
    world().run("scenarios/big_float_new_from_big_int.scen.json")?;

    Ok(())
}

#[test]
fn big_float_new_from_big_uint_go() -> anyhow::Result<()> {
    world().run("scenarios/big_float_new_from_big_uint.scen.json")?;

    Ok(())
}

#[test]
fn big_float_new_from_frac_go() -> anyhow::Result<()> {
    world().run("scenarios/big_float_new_from_frac.scen.json")?;

    Ok(())
}

#[test]
fn big_float_new_from_int_go() -> anyhow::Result<()> {
    world().run("scenarios/big_float_new_from_int.scen.json")?;

    Ok(())
}

#[test]
fn big_float_new_from_managed_buffer_go() -> anyhow::Result<()> {
    world().run("scenarios/big_float_new_from_managed_buffer.scen.json")?;

    Ok(())
}

#[test]
fn big_float_new_from_parts_go() -> anyhow::Result<()> {
    world().run("scenarios/big_float_new_from_parts.scen.json")?;

    Ok(())
}

#[test]
fn big_float_new_from_sci_go() -> anyhow::Result<()> {
    world().run("scenarios/big_float_new_from_sci.scen.json")?;

    Ok(())
}

#[test]
fn big_float_operator_checks_go() -> anyhow::Result<()> {
    world().run("scenarios/big_float_operator_checks.scen.json")?;

    Ok(())
}

#[test]
fn big_float_operators_go() -> anyhow::Result<()> {
    world().run("scenarios/big_float_operators.scen.json")?;

    Ok(())
}
