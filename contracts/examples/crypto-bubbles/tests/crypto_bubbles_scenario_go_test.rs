use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn balance_of_go() -> anyhow::Result<()> {
    world().run("scenarios/balanceOf.scen.json")?;

    Ok(())
}

#[test]
fn create_go() -> anyhow::Result<()> {
    world().run("scenarios/create.scen.json")?;

    Ok(())
}

#[test]
fn exceptions_go() -> anyhow::Result<()> {
    world().run("scenarios/exceptions.scen.json")?;

    Ok(())
}

#[test]
fn join_game_go() -> anyhow::Result<()> {
    world().run("scenarios/joinGame.scen.json")?;

    Ok(())
}

#[test]
fn reward_and_send_to_wallet_go() -> anyhow::Result<()> {
    world().run("scenarios/rewardAndSendToWallet.scen.json")?;

    Ok(())
}

#[test]
fn reward_winner_go() -> anyhow::Result<()> {
    world().run("scenarios/rewardWinner.scen.json")?;

    Ok(())
}

#[test]
fn reward_winner_last_go() -> anyhow::Result<()> {
    world().run("scenarios/rewardWinner_Last.scen.json")?;

    Ok(())
}

#[test]
fn top_up_ok_go() -> anyhow::Result<()> {
    world().run("scenarios/topUp_ok.scen.json")?;

    Ok(())
}

#[test]
fn top_up_withdraw_go() -> anyhow::Result<()> {
    world().run("scenarios/topUp_withdraw.scen.json")?;

    Ok(())
}

#[test]
fn withdraw_ok_go() -> anyhow::Result<()> {
    world().run("scenarios/withdraw_Ok.scen.json")?;

    Ok(())
}

#[test]
fn withdraw_too_much_go() -> anyhow::Result<()> {
    world().run("scenarios/withdraw_TooMuch.scen.json")?;

    Ok(())
}
