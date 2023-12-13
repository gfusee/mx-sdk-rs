use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn bid_first_go() -> anyhow::Result<()> {
    world().run("scenarios/bid_first.scen.json")?;

    Ok(())
}

#[test]
fn bid_second_max_go() -> anyhow::Result<()> {
    world().run("scenarios/bid_second_max.scen.json")?;

    Ok(())
}

#[test]
fn bid_second_ok_go() -> anyhow::Result<()> {
    world().run("scenarios/bid_second_ok.scen.json")?;

    Ok(())
}

#[test]
fn bid_second_too_low_go() -> anyhow::Result<()> {
    world().run("scenarios/bid_second_too_low.scen.json")?;

    Ok(())
}

#[test]
fn bid_siring_auction_go() -> anyhow::Result<()> {
    world().run("scenarios/bid_siring_auction.scen.json")?;

    Ok(())
}

#[test]
fn create_and_auction_gen_zero_kitty_go() -> anyhow::Result<()> {
    world().run("scenarios/create_and_auction_gen_zero_kitty.scen.json")?;

    Ok(())
}

#[test]
fn create_sale_auction_not_owner_go() -> anyhow::Result<()> {
    world().run("scenarios/create_sale_auction_not_owner.scen.json")?;

    Ok(())
}

#[test]
fn create_sale_auction_ok_go() -> anyhow::Result<()> {
    world().run("scenarios/create_sale_auction_ok.scen.json")?;

    Ok(())
}

#[test]
fn create_siring_auction_not_owner_go() -> anyhow::Result<()> {
    world().run("scenarios/create_siring_auction_not_owner.scen.json")?;

    Ok(())
}

#[test]
fn create_siring_auction_ok_go() -> anyhow::Result<()> {
    world().run("scenarios/create_siring_auction_ok.scen.json")?;

    Ok(())
}

#[test]
fn end_auction_no_bids_go() -> anyhow::Result<()> {
    world().run("scenarios/end_auction_no_bids.scen.json")?;

    Ok(())
}

#[test]
fn end_auction_second_bid_max_early_go() -> anyhow::Result<()> {
    world().run("scenarios/end_auction_second_bid_max_early.scen.json")?;

    Ok(())
}

#[test]
fn end_auction_second_bid_ok_early_go() -> anyhow::Result<()> {
    world().run("scenarios/end_auction_second_bid_ok_early.scen.json")?;

    Ok(())
}

#[test]
fn end_auction_second_bid_ok_late_go() -> anyhow::Result<()> {
    world().run("scenarios/end_auction_second_bid_ok_late.scen.json")?;

    Ok(())
}

#[test]
fn end_siring_auction_go() -> anyhow::Result<()> {
    world().run("scenarios/end_siring_auction.scen.json")?;

    Ok(())
}

#[test]
fn init_go() -> anyhow::Result<()> {
    world().run("scenarios/init.scen.json")?;

    Ok(())
}
