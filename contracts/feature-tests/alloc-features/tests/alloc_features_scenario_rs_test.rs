use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/alloc-features");

    blockchain.register_contract(
        "file:output/alloc-features.wasm",
        alloc_features::ContractBuilder,
    );

    blockchain
}
#[test]
fn boxed_bytes_zeros_rs() -> anyhow::Result<()> {
    world().run("scenarios/boxed_bytes_zeros.scen.json")?;

    Ok(())
}

#[test]
fn echo_async_result_empty_rs() -> anyhow::Result<()> {
    world().run("scenarios/echo_async_result_empty.scen.json")?;

    Ok(())
}

#[test]
fn echo_big_int_nested_alloc_rs() -> anyhow::Result<()> {
    world().run("scenarios/echo_big_int_nested_alloc.scen.json")?;

    Ok(())
}

#[test]
fn echo_boxed_bytes_rs() -> anyhow::Result<()> {
    world().run("scenarios/echo_boxed_bytes.scen.json")?;

    Ok(())
}

#[test]
fn echo_multi_value_tuples_alloc_rs() -> anyhow::Result<()> {
    world().run("scenarios/echo_multi_value_tuples_alloc.scen.json")?;

    Ok(())
}

#[test]
fn echo_ser_ex_1_rs() -> anyhow::Result<()> {
    world().run("scenarios/echo_ser_ex_1.scen.json")?;

    Ok(())
}

#[test]
fn echo_slice_u_8_rs() -> anyhow::Result<()> {
    world().run("scenarios/echo_slice_u8.scen.json")?;

    Ok(())
}

#[test]
fn echo_str_rs() -> anyhow::Result<()> {
    world().run("scenarios/echo_str.scen.json")?;

    Ok(())
}

#[test]
fn echo_str_box_rs() -> anyhow::Result<()> {
    world().run("scenarios/echo_str_box.scen.json")?;

    Ok(())
}

#[test]
fn echo_string_rs() -> anyhow::Result<()> {
    world().run("scenarios/echo_string.scen.json")?;

    Ok(())
}

#[test]
fn echo_varargs_u_32_alloc_rs() -> anyhow::Result<()> {
    world().run("scenarios/echo_varargs_u32_alloc.scen.json")?;

    Ok(())
}

#[test]
fn echo_vec_u_8_rs() -> anyhow::Result<()> {
    world().run("scenarios/echo_vec_u8.scen.json")?;

    Ok(())
}

#[test]
fn managed_buffer_concat_2_rs() -> anyhow::Result<()> {
    world().run("scenarios/managed_buffer_concat_2.scen.json")?;

    Ok(())
}

#[test]
fn managed_buffer_load_slice_rs() -> anyhow::Result<()> {
    world().run("scenarios/managed_buffer_load_slice.scen.json")?;

    Ok(())
}

#[test]
fn managed_buffer_overwrite_rs() -> anyhow::Result<()> {
    world().run("scenarios/managed_buffer_overwrite.scen.json")?;

    Ok(())
}

#[test]
fn managed_buffer_set_slice_rs() -> anyhow::Result<()> {
    world().run("scenarios/managed_buffer_set_slice.scen.json")?;

    Ok(())
}

#[test]
fn only_owner_legacy_rs() -> anyhow::Result<()> {
    world().run("scenarios/only_owner_legacy.scen.json")?;

    Ok(())
}

#[test]
fn sc_result_rs() -> anyhow::Result<()> {
    world().run("scenarios/sc_result.scen.json")?;

    Ok(())
}

#[test]
fn storage_address_rs() -> anyhow::Result<()> {
    world().run("scenarios/storage_address.scen.json")?;

    Ok(())
}

#[test]
fn storage_opt_address_rs() -> anyhow::Result<()> {
    world().run("scenarios/storage_opt_address.scen.json")?;

    Ok(())
}

#[test]
fn storage_vec_u_8_rs() -> anyhow::Result<()> {
    world().run("scenarios/storage_vec_u8.scen.json")?;

    Ok(())
}
