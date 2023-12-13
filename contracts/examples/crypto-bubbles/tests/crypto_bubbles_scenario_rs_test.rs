use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crypto-bubbles");

    blockchain.register_contract(
        "file:output/crypto-bubbles.wasm",
        crypto_bubbles::ContractBuilder,
    );
    blockchain
}

#[test]
fn balance_of_rs() -> anyhow::Result<()> {
    world().run("scenarios/balanceOf.scen.json")?;

    Ok(())
}

#[test]
fn create_rs() -> anyhow::Result<()> {
    world().run("scenarios/create.scen.json")?;

    Ok(())
}

#[test]
fn exceptions_rs() -> anyhow::Result<()> {
    world().run("scenarios/exceptions.scen.json")?;

    Ok(())
}

#[test]
fn join_game_rs() -> anyhow::Result<()> {
    world().run("scenarios/joinGame.scen.json")?;

    Ok(())
}

#[test]
fn reward_and_send_to_wallet_rs() -> anyhow::Result<()> {
    world().run("scenarios/rewardAndSendToWallet.scen.json")?;

    Ok(())
}

#[test]
fn reward_winner_rs() -> anyhow::Result<()> {
    world().run("scenarios/rewardWinner.scen.json")?;

    Ok(())
}

#[test]
fn reward_winner_last_rs() -> anyhow::Result<()> {
    world().run("scenarios/rewardWinner_Last.scen.json")?;

    Ok(())
}

#[test]
fn top_up_ok_rs() -> anyhow::Result<()> {
    world().run("scenarios/topUp_ok.scen.json")?;

    Ok(())
}

#[test]
fn top_up_withdraw_rs() -> anyhow::Result<()> {
    world().run("scenarios/topUp_withdraw.scen.json")?;

    Ok(())
}

#[test]
fn withdraw_ok_rs() -> anyhow::Result<()> {
    world().run("scenarios/withdraw_Ok.scen.json")?;

    Ok(())
}

#[test]
fn withdraw_too_much_rs() -> anyhow::Result<()> {
    world().run("scenarios/withdraw_TooMuch.scen.json")?;

    Ok(())
}
