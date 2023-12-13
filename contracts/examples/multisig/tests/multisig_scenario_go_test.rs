use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn call_other_shard_1_go() -> anyhow::Result<()> {
    world().run("scenarios/call_other_shard-1.scen.json")?;

    Ok(())
}

#[test]
fn call_other_shard_2_go() -> anyhow::Result<()> {
    world().run("scenarios/call_other_shard-2.scen.json")?;

    Ok(())
}

#[test]
fn change_board_go() -> anyhow::Result<()> {
    world().run("scenarios/changeBoard.scen.json")?;

    Ok(())
}

#[test]
fn change_quorum_go() -> anyhow::Result<()> {
    world().run("scenarios/changeQuorum.scen.json")?;

    Ok(())
}

#[test]
fn change_quorum_too_big_go() -> anyhow::Result<()> {
    world().run("scenarios/changeQuorum_tooBig.scen.json")?;

    Ok(())
}

#[test]
fn deploy_adder_err_go() -> anyhow::Result<()> {
    world().run("scenarios/deployAdder_err.scen.json")?;

    Ok(())
}

#[test]
fn deploy_adder_then_call_go() -> anyhow::Result<()> {
    world().run("scenarios/deployAdder_then_call.scen.json")?;

    Ok(())
}

#[test]
fn deploy_factorial_go() -> anyhow::Result<()> {
    world().run("scenarios/deployFactorial.scen.json")?;

    Ok(())
}

#[test]
fn deploy_other_multisig_go() -> anyhow::Result<()> {
    world().run("scenarios/deployOtherMultisig.scen.json")?;

    Ok(())
}

#[test]
fn deploy_duplicate_bm_go() -> anyhow::Result<()> {
    world().run("scenarios/deploy_duplicate_bm.scen.json")?;

    Ok(())
}

#[test]
#[ignore = "missing 'newTokenIdentifiers' syntax"]
fn interactor_nft_go() -> anyhow::Result<()> {
    world().run("scenarios/interactor_nft.scen.json")?;

    Ok(())
}

#[test]
#[ignore = "missing 'newTokenIdentifiers' syntax"]
fn interactor_nft_all_roles_go() -> anyhow::Result<()> {
    world().run("scenarios/interactor_nft_all_roles.scen.json")?;

    Ok(())
}

#[test]
fn interactor_wegld_go() -> anyhow::Result<()> {
    world().run("scenarios/interactor_wegld.scen.json")?;

    Ok(())
}

#[test]
fn remove_everyone_go() -> anyhow::Result<()> {
    world().run("scenarios/remove_everyone.scen.json")?;

    Ok(())
}

// TODO: investigate gas issue
#[test]
#[ignore]
fn send_esdt_go() -> anyhow::Result<()> {
    world().run("scenarios/sendEsdt.scen.json")?;

    Ok(())
}

#[test]
fn upgrade_go() -> anyhow::Result<()> {
    world().run("scenarios/upgrade.scen.json")?;

    Ok(())
}

#[test]
fn upgrade_from_source_go() -> anyhow::Result<()> {
    world().run("scenarios/upgrade_from_source.scen.json")?;

    Ok(())
}
