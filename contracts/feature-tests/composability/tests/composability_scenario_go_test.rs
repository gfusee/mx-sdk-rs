use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn builtin_func_delete_user_name_go() -> anyhow::Result<()> {
    world().run("scenarios/builtin_func_delete_user_name.scen.json")?;
    
    Ok(())
}

#[test]
fn builtin_func_set_user_name_go() -> anyhow::Result<()> {
    world().run("scenarios/builtin_func_set_user_name.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_queue_async_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_queue_async.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_async_accept_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_async_accept_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_async_accept_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_async_accept_esdt.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_async_echo_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_async_echo.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_async_send_and_retrieve_multi_transfer_funds_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_async_send_and_retrieve_multi_transfer_funds.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_builtin_nft_local_mint_via_async_call_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_builtin_nft_local_mint_via_async_call.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_builtin_nft_local_mint_via_sync_call_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_builtin_nft_local_mint_via_sync_call.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_call_async_retrieve_multi_transfer_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_call_async_retrieve_multi_transfer.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_contract_deploy_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_contract_deploy.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_contract_upgrade_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_contract_upgrade.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_contract_upgrade_self_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_contract_upgrade_self.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_direct_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_direct_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_direct_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_direct_esdt.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_direct_multi_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_direct_multi_esdt.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_init_async_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_init_async.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_init_sync_accept_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_init_sync_accept_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_init_sync_echo_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_init_sync_echo.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_sync_echo_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_sync_echo.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_sync_echo_caller_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_sync_echo_caller.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_sync_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_sync_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_sync_readonly_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_sync_readonly.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_sync_same_context_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_sync_same_context.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_sync_same_context_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_sync_same_context_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_transf_exec_accept_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_transf_exec_accept_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn forw_raw_transf_exec_reject_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/forw_raw_transf_exec_reject_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_builtin_nft_add_quantity_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_builtin_nft_add_quantity.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_builtin_nft_burn_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_builtin_nft_burn.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_builtin_nft_create_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_builtin_nft_create.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_builtin_nft_local_burn_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_builtin_nft_local_burn.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_builtin_nft_local_mint_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_builtin_nft_local_mint.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_async_accept_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_async_accept_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_async_accept_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_async_accept_esdt.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_async_accept_nft_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_async_accept_nft.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_async_multi_transfer_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_async_multi_transfer.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_async_retrieve_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_async_retrieve_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_async_retrieve_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_async_retrieve_esdt.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_async_retrieve_nft_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_async_retrieve_nft.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_sync_accept_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_sync_accept_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_sync_accept_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_sync_accept_esdt.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_sync_accept_multi_transfer_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_sync_accept_multi_transfer.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_sync_accept_nft_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_sync_accept_nft.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_sync_accept_then_read_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_sync_accept_then_read_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_sync_accept_then_read_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_sync_accept_then_read_esdt.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_sync_accept_then_read_nft_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_sync_accept_then_read_nft.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_sync_retrieve_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_sync_retrieve_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_sync_retrieve_egld_bt_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_sync_retrieve_egld_bt.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_sync_retrieve_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_sync_retrieve_esdt.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_sync_retrieve_esdt_bt_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_sync_retrieve_esdt_bt.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_sync_retrieve_nft_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_sync_retrieve_nft.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_sync_retrieve_nft_bt_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_sync_retrieve_nft_bt.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_transf_exec_accept_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_transf_exec_accept_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_transf_exec_accept_egld_twice_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_transf_exec_accept_egld_twice.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_transf_exec_accept_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_transf_exec_accept_esdt.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_transf_exec_accept_esdt_twice_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_transf_exec_accept_esdt_twice.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_transf_exec_accept_multi_transfer_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_transf_exec_accept_multi_transfer.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_transf_exec_accept_nft_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_transf_exec_accept_nft.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_transf_exec_accept_return_values_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_transf_exec_accept_return_values.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_transf_exec_accept_sft_twice_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_transf_exec_accept_sft_twice.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_transf_exec_reject_multi_transfer_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_transf_exec_reject_multi_transfer.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_call_transf_exec_reject_nft_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_call_transf_exec_reject_nft.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_contract_change_owner_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_contract_change_owner.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_contract_deploy_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_contract_deploy.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_contract_upgrade_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_contract_upgrade.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_get_esdt_local_roles_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_get_esdt_local_roles.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_get_esdt_token_data_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_get_esdt_token_data.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_nft_add_uri_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_nft_add_uri.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_nft_create_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_nft_create.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_nft_create_and_send_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_nft_create_and_send.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_nft_current_nonce_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_nft_current_nonce.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_nft_decode_complex_attributes_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_nft_decode_complex_attributes.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_nft_transfer_async_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_nft_transfer_async.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_nft_transfer_exec_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_nft_transfer_exec.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_nft_update_attributes_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_nft_update_attributes.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_no_endpoint_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_no_endpoint.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_retrieve_funds_with_accept_func_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_retrieve_funds_with_accept_func.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_send_esdt_multi_transfer_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_send_esdt_multi_transfer.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_sync_echo_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_sync_echo.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_tranfer_esdt_with_fees_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_tranfer_esdt_with_fees.scen.json")?;
    
    Ok(())
}

#[test]
fn forwarder_validate_token_identifier_go() -> anyhow::Result<()> {
    world().run("scenarios/forwarder_validate_token_identifier.scen.json")?;
    
    Ok(())
}

#[test]
fn promises_call_async_accept_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/promises_call_async_accept_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn promises_call_async_accept_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/promises_call_async_accept_esdt.scen.json")?;
    
    Ok(())
}

#[test]
#[ignore = "TODO"]
fn promises_call_async_retrieve_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/promises_call_async_retrieve_egld.scen.json")?;
    
    Ok(())
}

#[test]
#[ignore = "TODO"]
fn promises_call_async_retrieve_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/promises_call_async_retrieve_esdt.scen.json")?;
    
    Ok(())
}

#[test]
#[ignore = "TODO"]
fn promises_call_callback_directly_go() -> anyhow::Result<()> {
    world().run("scenarios/promises_call_callback_directly.scen.json")?;
    
    Ok(())
}

#[test]
#[ignore = "TODO"]
fn promises_multi_transfer_go() -> anyhow::Result<()> {
    world().run("scenarios/promises_multi_transfer.scen.json")?;
    
    Ok(())
}

#[test]
fn promises_single_transfer_go() -> anyhow::Result<()> {
    world().run("scenarios/promises_single_transfer.scen.json")?;
    
    Ok(())
}

#[test]
#[ignore = "gas"]
fn promises_single_transfer_gas_1_go() -> anyhow::Result<()> {
    world().run("scenarios/promises_single_transfer_gas1.scen.json")?;
    
    Ok(())
}

#[test]
#[ignore = "gas"]
fn promises_single_transfer_gas_2_go() -> anyhow::Result<()> {
    world().run("scenarios/promises_single_transfer_gas2.scen.json")?;
    
    Ok(())
}

#[test]
fn proxy_test_init_go() -> anyhow::Result<()> {
    world().run("scenarios/proxy_test_init.scen.json")?;
    
    Ok(())
}

#[test]
fn proxy_test_message_other_shard_go() -> anyhow::Result<()> {
    world().run("scenarios/proxy_test_message_otherShard.scen.json")?;
    
    Ok(())
}

#[test]
fn proxy_test_message_other_shard_callback_go() -> anyhow::Result<()> {
    world().run("scenarios/proxy_test_message_otherShard_callback.scen.json")?;
    
    Ok(())
}

#[test]
fn proxy_test_message_same_shard_go() -> anyhow::Result<()> {
    world().run("scenarios/proxy_test_message_sameShard.scen.json")?;
    
    Ok(())
}

#[test]
fn proxy_test_message_same_shard_callback_go() -> anyhow::Result<()> {
    world().run("scenarios/proxy_test_message_sameShard_callback.scen.json")?;
    
    Ok(())
}

#[test]
fn proxy_test_payment_other_shard_go() -> anyhow::Result<()> {
    world().run("scenarios/proxy_test_payment_otherShard.scen.json")?;
    
    Ok(())
}

#[test]
fn proxy_test_payment_other_shard_callback_go() -> anyhow::Result<()> {
    world().run("scenarios/proxy_test_payment_otherShard_callback.scen.json")?;
    
    Ok(())
}

#[test]
fn proxy_test_payment_same_shard_go() -> anyhow::Result<()> {
    world().run("scenarios/proxy_test_payment_sameShard.scen.json")?;
    
    Ok(())
}

#[test]
fn proxy_test_payment_same_shard_callback_go() -> anyhow::Result<()> {
    world().run("scenarios/proxy_test_payment_sameShard_callback.scen.json")?;
    
    Ok(())
}

#[test]
fn proxy_test_upgrade_go() -> anyhow::Result<()> {
    world().run("scenarios/proxy_test_upgrade.scen.json")?;
    
    Ok(())
}

#[test]
fn recursive_caller_egld_1_go() -> anyhow::Result<()> {
    world().run("scenarios/recursive_caller_egld_1.scen.json")?;
    
    Ok(())
}

#[test]
fn recursive_caller_esdt_1_go() -> anyhow::Result<()> {
    world().run("scenarios/recursive_caller_esdt_1.scen.json")?;
    
    Ok(())
}

#[test]
fn send_egld_go() -> anyhow::Result<()> {
    world().run("scenarios/send_egld.scen.json")?;
    
    Ok(())
}

#[test]
fn send_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/send_esdt.scen.json")?;
    
    Ok(())
}
