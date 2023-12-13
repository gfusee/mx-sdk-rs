use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn big_int_from_i_64_go() -> anyhow::Result<()> {
    world().run("scenarios/big_int_from_i64.scen.json")?;
    
    Ok(())
}

#[test]
fn big_int_to_i_64_go() -> anyhow::Result<()> {
    world().run("scenarios/big_int_to_i64.scen.json")?;
    
    Ok(())
}

#[test]
fn big_num_conversions_go() -> anyhow::Result<()> {
    world().run("scenarios/big_num_conversions.scen.json")?;
    
    Ok(())
}

#[test]
fn big_uint_eq_u_64_go() -> anyhow::Result<()> {
    world().run("scenarios/big_uint_eq_u64.scen.json")?;
    
    Ok(())
}

#[test]
fn big_uint_from_u_64_go() -> anyhow::Result<()> {
    world().run("scenarios/big_uint_from_u64.scen.json")?;
    
    Ok(())
}

#[test]
fn big_uint_pow_go() -> anyhow::Result<()> {
    world().run("scenarios/big_uint_pow.scen.json")?;
    
    Ok(())
}

#[test]
fn big_uint_sqrt_go() -> anyhow::Result<()> {
    world().run("scenarios/big_uint_sqrt.scen.json")?;
    
    Ok(())
}

#[test]
fn big_uint_to_u_64_go() -> anyhow::Result<()> {
    world().run("scenarios/big_uint_to_u64.scen.json")?;
    
    Ok(())
}

#[test]
fn block_info_go() -> anyhow::Result<()> {
    world().run("scenarios/block_info.scen.json")?;
    
    Ok(())
}

#[test]
fn codec_err_go() -> anyhow::Result<()> {
    world().run("scenarios/codec_err.scen.json")?;
    
    Ok(())
}

#[test]
fn count_ones_go() -> anyhow::Result<()> {
    world().run("scenarios/count_ones.scen.json")?;
    
    Ok(())
}

#[test]
fn crypto_elliptic_curves_go() -> anyhow::Result<()> {
    world().run("scenarios/crypto_elliptic_curves.scen.json")?;
    
    Ok(())
}

#[test]
fn crypto_keccak_256_go() -> anyhow::Result<()> {
    world().run("scenarios/crypto_keccak256.scen.json")?;
    
    Ok(())
}

#[test]
fn crypto_ripemd_160_go() -> anyhow::Result<()> {
    world().run("scenarios/crypto_ripemd160.scen.json")?;
    
    Ok(())
}

#[test]
fn crypto_sha_256_go() -> anyhow::Result<()> {
    world().run("scenarios/crypto_sha256.scen.json")?;
    
    Ok(())
}

#[test]
fn crypto_verify_bls_go() -> anyhow::Result<()> {
    world().run("scenarios/crypto_verify_bls.scen.json")?;
    
    Ok(())
}

#[test]
fn crypto_verify_ed_25519_go() -> anyhow::Result<()> {
    world().run("scenarios/crypto_verify_ed25519.scen.json")?;
    
    Ok(())
}

#[test]
fn crypto_verify_secp_256_k_1_go() -> anyhow::Result<()> {
    world().run("scenarios/crypto_verify_secp256k1.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_array_u_8_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_array_u8.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_arrayvec_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_arrayvec.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_big_int_nested_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_big_int_nested.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_big_int_top_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_big_int_top.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_big_uint_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_big_uint.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_i_32_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_i32.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_i_64_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_i64.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_ignore_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_ignore.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_managed_async_result_empty_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_managed_async_result_empty.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_managed_bytes_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_managed_bytes.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_managed_vec_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_managed_vec.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_multi_value_tuples_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_multi_value_tuples.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_nothing_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_nothing.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_tuple_into_multiresult_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_tuple_into_multiresult.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_u_64_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_u64.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_usize_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_usize.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_varargs_managed_eager_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_varargs_managed_eager.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_varargs_managed_sum_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_varargs_managed_sum.scen.json")?;
    
    Ok(())
}

#[test]
fn echo_varargs_u_32_go() -> anyhow::Result<()> {
    world().run("scenarios/echo_varargs_u32.scen.json")?;
    
    Ok(())
}

#[test]
fn events_go() -> anyhow::Result<()> {
    world().run("scenarios/events.scen.json")?;
    
    Ok(())
}

#[test]
fn get_caller_go() -> anyhow::Result<()> {
    world().run("scenarios/get_caller.scen.json")?;
    
    Ok(())
}

#[test]
fn get_cumulated_validator_rewards_go() -> anyhow::Result<()> {
    world().run("scenarios/get_cumulated_validator_rewards.scen.json")?;
    
    Ok(())
}

#[test]
#[ignore = "TODO: missing support from scenario-go"]
fn get_shard_of_address_go() -> anyhow::Result<()> {
    world().run("scenarios/get_shard_of_address.scen.json")?;
    
    Ok(())
}

#[test]
fn managed_address_array_go() -> anyhow::Result<()> {
    world().run("scenarios/managed_address_array.scen.json")?;
    
    Ok(())
}

#[test]
fn managed_address_managed_buffer_go() -> anyhow::Result<()> {
    world().run("scenarios/managed_address_managed_buffer.scen.json")?;
    
    Ok(())
}

#[test]
fn managed_buffer_concat_go() -> anyhow::Result<()> {
    world().run("scenarios/managed_buffer_concat.scen.json")?;
    
    Ok(())
}

#[test]
fn managed_buffer_copy_slice_go() -> anyhow::Result<()> {
    world().run("scenarios/managed_buffer_copy_slice.scen.json")?;
    
    Ok(())
}

#[test]
fn managed_buffer_eq_go() -> anyhow::Result<()> {
    world().run("scenarios/managed_buffer_eq.scen.json")?;
    
    Ok(())
}

#[test]
fn managed_buffer_set_random_go() -> anyhow::Result<()> {
    world().run("scenarios/managed_buffer_set_random.scen.json")?;
    
    Ok(())
}

#[test]
fn managed_vec_address_push_go() -> anyhow::Result<()> {
    world().run("scenarios/managed_vec_address_push.scen.json")?;
    
    Ok(())
}

#[test]
fn managed_vec_array_push_go() -> anyhow::Result<()> {
    world().run("scenarios/managed_vec_array_push.scen.json")?;
    
    Ok(())
}

#[test]
fn managed_vec_biguint_push_go() -> anyhow::Result<()> {
    world().run("scenarios/managed_vec_biguint_push.scen.json")?;
    
    Ok(())
}

#[test]
fn only_owner_go() -> anyhow::Result<()> {
    world().run("scenarios/only_owner.scen.json")?;
    
    Ok(())
}

#[test]
fn only_user_account_go() -> anyhow::Result<()> {
    world().run("scenarios/only_user_account.scen.json")?;
    
    Ok(())
}

#[test]
fn out_of_gas_go() -> anyhow::Result<()> {
    world().run("scenarios/out_of_gas.scen.json")?;
    
    Ok(())
}

#[test]
fn panic_go() -> anyhow::Result<()> {
    world().run("scenarios/panic.scen.json")?;
    
    Ok(())
}

#[test]
fn return_codes_go() -> anyhow::Result<()> {
    world().run("scenarios/return_codes.scen.json")?;
    
    Ok(())
}

#[test]
fn sc_properties_go() -> anyhow::Result<()> {
    world().run("scenarios/sc_properties.scen.json")?;
    
    Ok(())
}

#[test]
fn small_num_overflow_go() -> anyhow::Result<()> {
    world().run("scenarios/small_num_overflow.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_big_int_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_big_int.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_big_uint_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_big_uint.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_bool_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_bool.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_clear_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_clear.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_i_64_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_i64.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_i_64_bad_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_i64_bad.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_load_from_address_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_load_from_address.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_managed_address_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_managed_address.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_map_1_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_map1.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_map_2_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_map2.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_map_3_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_map3.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_mapper_address_to_id_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_mapper_address_to_id.scen.json")?;
    
    Ok(())
}

#[test]
#[ignore = "currently not supported. TODO: investigate"]
fn storage_mapper_fungible_token_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_mapper_fungible_token.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_mapper_linked_list_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_mapper_linked_list.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_mapper_map_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_mapper_map.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_mapper_map_storage_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_mapper_map_storage.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_mapper_non_fungible_token_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_mapper_non_fungible_token.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_mapper_queue_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_mapper_queue.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_mapper_set_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_mapper_set.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_mapper_single_value_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_mapper_single_value.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_mapper_token_attributes_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_mapper_token_attributes.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_mapper_unique_id_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_mapper_unique_id.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_mapper_vec_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_mapper_vec.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_mapper_whitelist_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_mapper_whitelist.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_opt_managed_addr_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_opt_managed_addr.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_raw_api_features_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_raw_api_features.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_reserved_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_reserved.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_u_64_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_u64.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_u_64_bad_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_u64_bad.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_usize_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_usize.scen.json")?;
    
    Ok(())
}

#[test]
fn storage_usize_bad_go() -> anyhow::Result<()> {
    world().run("scenarios/storage_usize_bad.scen.json")?;
    
    Ok(())
}

#[test]
fn struct_eq_go() -> anyhow::Result<()> {
    world().run("scenarios/struct_eq.scen.json")?;
    
    Ok(())
}
