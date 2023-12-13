use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn generated_fund_go() -> anyhow::Result<()> {
    world().run("scenarios/_generated_fund.scen.json")?;

    Ok(())
}

#[test]
fn generated_init_go() -> anyhow::Result<()> {
    world().run("scenarios/_generated_init.scen.json")?;

    Ok(())
}

#[test]
fn generated_query_status_go() -> anyhow::Result<()> {
    world().run("scenarios/_generated_query_status.scen.json")?;

    Ok(())
}

#[test]
fn generated_sc_err_go() -> anyhow::Result<()> {
    world().run("scenarios/_generated_sc_err.scen.json")?;

    Ok(())
}

#[test]
fn crowdfunding_claim_failed_go() -> anyhow::Result<()> {
    world().run("scenarios/crowdfunding-claim-failed.scen.json")?;

    Ok(())
}

#[test]
fn crowdfunding_claim_successful_go() -> anyhow::Result<()> {
    world().run("scenarios/crowdfunding-claim-successful.scen.json")?;

    Ok(())
}

#[test]
fn crowdfunding_claim_too_early_go() -> anyhow::Result<()> {
    world().run("scenarios/crowdfunding-claim-too-early.scen.json")?;

    Ok(())
}

#[test]
fn crowdfunding_fund_go() -> anyhow::Result<()> {
    world().run("scenarios/crowdfunding-fund.scen.json")?;

    Ok(())
}

#[test]
fn crowdfunding_fund_too_late_go() -> anyhow::Result<()> {
    world().run("scenarios/crowdfunding-fund-too-late.scen.json")?;

    Ok(())
}

#[test]
fn crowdfunding_init_go() -> anyhow::Result<()> {
    world().run("scenarios/crowdfunding-init.scen.json")?;

    Ok(())
}

#[test]
fn egld_crowdfunding_claim_failed_go() -> anyhow::Result<()> {
    world().run("scenarios/egld-crowdfunding-claim-failed.scen.json")?;

    Ok(())
}

#[test]
fn egld_crowdfunding_claim_successful_go() -> anyhow::Result<()> {
    world().run("scenarios/egld-crowdfunding-claim-successful.scen.json")?;

    Ok(())
}

#[test]
fn egld_crowdfunding_claim_too_early_go() -> anyhow::Result<()> {
    world().run("scenarios/egld-crowdfunding-claim-too-early.scen.json")?;

    Ok(())
}

#[test]
fn egld_crowdfunding_fund_go() -> anyhow::Result<()> {
    world().run("scenarios/egld-crowdfunding-fund.scen.json")?;

    Ok(())
}

#[test]
fn egld_crowdfunding_fund_too_late_go() -> anyhow::Result<()> {
    world().run("scenarios/egld-crowdfunding-fund-too-late.scen.json")?;

    Ok(())
}

#[test]
fn egld_crowdfunding_init_go() -> anyhow::Result<()> {
    world().run("scenarios/egld-crowdfunding-init.scen.json")?;

    Ok(())
}
