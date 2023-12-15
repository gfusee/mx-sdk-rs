mod user_builtin {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait UserBuiltin {
        #[endpoint(SetUserName)]
        fn set_user_name(&self, name: &BoxedBytes) -> BigUint;
    }
}

mod dns_mock {
    multiversx_sc::imports!();

    #[multiversx_sc::contract]
    pub trait DnsMock {
        #[proxy]
        fn user_builtin_proxy(&self, to: ManagedAddress) -> super::user_builtin::Proxy<Self::Api>;

        #[payable("EGLD")]
        #[endpoint]
        fn register(&self, name: BoxedBytes) {
            let _payment = self.call_value().egld_value();
            let address = self.blockchain().get_caller();
            self.user_builtin_proxy(address)
                .set_user_name(&name)
                .async_call()
                .call_and_exit()
        }
    }
}

use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract("file:output/use-module.wasm", use_module::ContractBuilder);

    blockchain.register_contract(
        "file:test-wasm/elrond-wasm-sc-dns.wasm",
        dns_mock::ContractBuilder,
    );

    blockchain
}

#[test]
fn use_module_claim_developer_rewards_rs() -> anyhow::Result<()> {
    world().run("scenarios/use_module_claim_developer_rewards.scen.json")?;

    Ok(())
}

#[test]
fn use_module_dns_register_rs() -> anyhow::Result<()> {
    world().run("scenarios/use_module_dns_register.scen.json")?;

    Ok(())
}

#[test]
fn use_module_features_rs() -> anyhow::Result<()> {
    world().run("scenarios/use_module_features.scen.json")?;

    Ok(())
}

#[test]
fn use_module_internal_rs() -> anyhow::Result<()> {
    world().run("scenarios/use_module_internal.scen.json")?;

    Ok(())
}

#[test]
fn use_module_no_endpoint_rs() -> anyhow::Result<()> {
    world().run("scenarios/use_module_no_endpoint.scen.json")?;

    Ok(())
}

/// Will not work in scenarios-rs, since there is no gas usage
#[test]
#[ignore]
fn use_module_ongoing_operation_example_rs() -> anyhow::Result<()> {
    world().run("scenarios/use_module_ongoing_operation_example.scen.json")?;

    Ok(())
}

#[test]
fn use_module_only_admin_rs() -> anyhow::Result<()> {
    world().run("scenarios/use_module_only_admin.scen.json")?;

    Ok(())
}

#[test]
fn use_module_only_owner_rs() -> anyhow::Result<()> {
    world().run("scenarios/use_module_only_owner.scen.json")?;

    Ok(())
}

#[test]
fn use_module_pause_rs() -> anyhow::Result<()> {
    world().run("scenarios/use_module_pause.scen.json")?;

    Ok(())
}
