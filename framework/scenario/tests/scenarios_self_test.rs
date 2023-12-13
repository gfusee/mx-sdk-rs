use multiversx_sc_scenario::*;
use multiversx_sc_scenario::scenario_model::ScCallStep;

// These tests don't really test any contract, but the testing framework itslef.

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("framework/scenario");
    blockchain
}

/// Checks that externalSteps work fine.
#[test]
fn external_steps_rs() {
    world().run("tests/scenarios-self/external_steps/external_steps.scen.json");
}

#[test]
#[should_panic]
fn set_account_addr_len_err1_rs() {
    world().run("tests/scenarios-self/set-check/set-account-addr-len.err1.json");
}

#[test]
#[should_panic]
fn set_account_addr_len_err2_rs() {
    world().run("tests/scenarios-self/set-check/set-account-addr-len.err2.json");
}

#[test]
#[should_panic]
fn set_account_sc_addr_err1_rs() {
    world().run("tests/scenarios-self/set-check/set-account-sc-addr.err1.json");
}

#[test]
#[should_panic]
fn set_account_sc_addr_err2_rs() {
    world().run("tests/scenarios-self/set-check/set-account-sc-addr.err2.json");
}

#[test]
#[should_panic]
fn set_account_sc_addr_err3_rs() {
    world().run("tests/scenarios-self/set-check/set-account-sc-addr.err3.json");
}

#[test]
#[should_panic]
fn set_check_balance_err_rs() {
    world().run("tests/scenarios-self/set-check/set-check-balance.err.json");
}

#[test]
fn set_check_balance_rs() {
    world().run("tests/scenarios-self/set-check/set-check-balance.scen.json");
}

#[test]
#[should_panic]
fn set_check_code_err_rs() {
    world().run("tests/scenarios-self/set-check/set-check-code.err.json");
}

#[test]
fn set_check_code() {
    world().run("tests/scenarios-self/set-check/set-check-code.scen.json");
}

#[test]
#[should_panic]
fn set_check_esdt_err_rs() {
    world().run("tests/scenarios-self/set-check/set-check-esdt.err1.json");
}

#[test]
fn set_check_esdt_rs() {
    world().run("tests/scenarios-self/set-check/set-check-esdt.scen.json");
}

#[test]
#[should_panic]
fn set_check_nonce_err_rs() {
    world().run("tests/scenarios-self/set-check/set-check-nonce.err.json");
}

#[test]
fn set_check_nonce_rs() {
    world().run("tests/scenarios-self/set-check/set-check-nonce.scen.json");
}

#[test]
#[should_panic]
fn set_check_storage_err1_rs() {
    world().run("tests/scenarios-self/set-check/set-check-storage.err1.json");
}

#[test]
#[should_panic]
fn set_check_storage_err2_rs() {
    world().run("tests/scenarios-self/set-check/set-check-storage.err2.json");
}

#[test]
#[should_panic]
fn set_check_storage_err3_rs() {
    world().run("tests/scenarios-self/set-check/set-check-storage.err3.json");
}

#[test]
#[should_panic]
fn set_check_storage_err4_rs() {
    world().run("tests/scenarios-self/set-check/set-check-storage.err4.json");
}

#[test]
#[should_panic]
fn set_check_storage_err5_rs() {
    world().run("tests/scenarios-self/set-check/set-check-storage.err5.json");
}

#[test]
fn set_check_storage_rs() {
    world().run("tests/scenarios-self/set-check/set-check-storage.scen.json");
}

#[test]
#[should_panic]
fn set_check_username_err_rs() {
    world().run("tests/scenarios-self/set-check/set-check-username.err.json");
}

#[test]
fn set_check_username_rs() {
    world().run("tests/scenarios-self/set-check/set-check-username.scen.json");
}

#[test]
fn builtin_func_esdt_transfer() {
    world().run("tests/scenarios-self/builtin-func-esdt-transfer.scen.json");
}

#[test]
#[should_panic]
fn esdt_non_zero_balance_check_err_rs() {
    world().run("tests/scenarios-self/esdt-non-zero-balance-check-err.scen.json");
}

#[test]
#[should_panic]
fn esdt_zero_balance_check_err_rs() {
    world().run("tests/scenarios-self/esdt-zero-balance-check-err.scen.json");
}

#[test]
fn multi_transfer_esdt_rs() {
    world().run("tests/scenarios-self/multi-transfer-esdt.scen.json");
}

#[test]
fn transfer_egld_rs() {
    world().run("tests/scenarios-self/transfer-egld.scen.json");
}

#[test]
fn transfer_esdt_rs() {
    world().run("tests/scenarios-self/transfer-esdt.scen.json");
}

#[test]
fn validator_reward_rs() {
    world().run("tests/scenarios-self/validatorReward.scen.json");
}

#[test]
fn call_unknown_contract() {
    let error = world().run("tests/scenarios-self/call-unknown-contract-err.scen.json")
        .unwrap_err();

    assert_eq!(error.to_string(), "Account 0x00000000000000000000756e6b6e6f776e5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f not found");
}
