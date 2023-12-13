use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn buy_all_tickets_different_accounts_go() -> anyhow::Result<()> {
    world().run("scenarios/buy-all-tickets-different-accounts.scen.json")?;
    
    Ok(())
}

#[test]
fn buy_more_tickets_than_allowed_go() -> anyhow::Result<()> {
    world().run("scenarios/buy-more-tickets-than-allowed.scen.json")?;
    
    Ok(())
}

#[test]
fn buy_ticket_go() -> anyhow::Result<()> {
    world().run("scenarios/buy-ticket.scen.json")?;
    
    Ok(())
}

#[test]
fn buy_ticket_after_deadline_go() -> anyhow::Result<()> {
    world().run("scenarios/buy-ticket-after-deadline.scen.json")?;
    
    Ok(())
}

#[test]
fn buy_ticket_after_determined_winner_go() -> anyhow::Result<()> {
    world().run("scenarios/buy-ticket-after-determined-winner.scen.json")?;
    
    Ok(())
}

#[test]
fn buy_ticket_after_sold_out_go() -> anyhow::Result<()> {
    world().run("scenarios/buy-ticket-after-sold-out.scen.json")?;
    
    Ok(())
}

#[test]
fn buy_ticket_all_options_go() -> anyhow::Result<()> {
    world().run("scenarios/buy-ticket-all-options.scen.json")?;
    
    Ok(())
}

#[test]
fn buy_ticket_another_account_go() -> anyhow::Result<()> {
    world().run("scenarios/buy-ticket-another-account.scen.json")?;
    
    Ok(())
}

#[test]
fn buy_ticket_not_on_whitelist_go() -> anyhow::Result<()> {
    world().run("scenarios/buy-ticket-not-on-whitelist.scen.json")?;
    
    Ok(())
}

#[test]
fn buy_ticket_same_account_go() -> anyhow::Result<()> {
    world().run("scenarios/buy-ticket-same-account.scen.json")?;
    
    Ok(())
}

#[test]
fn buy_ticket_second_lottery_go() -> anyhow::Result<()> {
    world().run("scenarios/buy-ticket-second-lottery.scen.json")?;
    
    Ok(())
}

#[test]
fn buy_ticket_wrong_fee_go() -> anyhow::Result<()> {
    world().run("scenarios/buy-ticket-wrong-fee.scen.json")?;
    
    Ok(())
}

#[test]
fn complex_prize_distribution_go() -> anyhow::Result<()> {
    world().run("scenarios/complex-prize-distribution.scen.json")?;
    
    Ok(())
}

#[test]
fn determine_winner_different_ticket_holders_winner_acc_1_go() -> anyhow::Result<()> {
    world().run("scenarios/determine-winner-different-ticket-holders-winner-acc1.scen.json")?;
    
    Ok(())
}

#[test]
fn determine_winner_early_go() -> anyhow::Result<()> {
    world().run("scenarios/determine-winner-early.scen.json")?;
    
    Ok(())
}

#[test]
fn determine_winner_same_ticket_holder_go() -> anyhow::Result<()> {
    world().run("scenarios/determine-winner-same-ticket-holder.scen.json")?;
    
    Ok(())
}

#[test]
fn determine_winner_split_prize_pool_go() -> anyhow::Result<()> {
    world().run("scenarios/determine-winner-split-prize-pool.scen.json")?;
    
    Ok(())
}

#[test]
fn lottery_init_go() -> anyhow::Result<()> {
    world().run("scenarios/lottery-init.scen.json")?;
    
    Ok(())
}

#[test]
fn lottery_with_burn_percentage_go() -> anyhow::Result<()> {
    world().run("scenarios/lottery-with-burn-percentage.scen.json")?;
    
    Ok(())
}

#[test]
fn start_after_announced_winner_go() -> anyhow::Result<()> {
    world().run("scenarios/start-after-announced-winner.scen.json")?;
    
    Ok(())
}

#[test]
fn start_all_options_bigger_whitelist_go() -> anyhow::Result<()> {
    world().run("scenarios/start-all-options-bigger-whitelist.scen.json")?;
    
    Ok(())
}

#[test]
fn start_alternative_function_name_go() -> anyhow::Result<()> {
    world().run("scenarios/start-alternative-function-name.scen.json")?;
    
    Ok(())
}

#[test]
fn start_fixed_deadline_go() -> anyhow::Result<()> {
    world().run("scenarios/start-fixed-deadline.scen.json")?;
    
    Ok(())
}

#[test]
fn start_limited_tickets_go() -> anyhow::Result<()> {
    world().run("scenarios/start-limited-tickets.scen.json")?;
    
    Ok(())
}

#[test]
fn start_limited_tickets_and_fixed_deadline_go() -> anyhow::Result<()> {
    world().run("scenarios/start-limited-tickets-and-fixed-deadline.scen.json")?;
    
    Ok(())
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_deadline_go() -> anyhow::Result<()> {
    world().run("scenarios/start-limited-tickets-and-fixed-deadline-invalid-deadline.scen.json")?;
    
    Ok(())
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_ticket_price_arg_go() -> anyhow::Result<()> {
    world().run(
        "scenarios/start-limited-tickets-and-fixed-deadline-invalid-ticket-price-arg.scen.json",
    )?;

    Ok(())
}

#[test]
fn start_second_lottery_go() -> anyhow::Result<()> {
    world().run("scenarios/start-second-lottery.scen.json")?;
    
    Ok(())
}

#[test]
fn start_with_all_options_go() -> anyhow::Result<()> {
    world().run("scenarios/start-with-all-options.scen.json")?;
    
    Ok(())
}

#[test]
fn start_with_no_options_go() -> anyhow::Result<()> {
    world().run("scenarios/start-with-no-options.scen.json")?;
    
    Ok(())
}
