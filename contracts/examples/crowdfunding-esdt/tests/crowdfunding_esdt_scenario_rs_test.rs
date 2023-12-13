use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-esdt");

    blockchain.register_contract(
        "file:output/crowdfunding-esdt.wasm",
        crowdfunding_esdt::ContractBuilder,
    );
    blockchain
}

#[test]
fn generated_fund_rs() -> anyhow::Result<()> {
    world().run("scenarios/_generated_fund.scen.json")?;

    Ok(())
}

#[test]
fn generated_init_rs() -> anyhow::Result<()> {
    world().run("scenarios/_generated_init.scen.json")?;

    Ok(())
}

#[test]
fn generated_query_status_rs() -> anyhow::Result<()> {
    world().run("scenarios/_generated_query_status.scen.json")?;

    Ok(())
}

#[test]
fn generated_sc_err_rs() -> anyhow::Result<()> {
    world().run("scenarios/_generated_sc_err.scen.json")?;

    Ok(())
}

#[test]
fn crowdfunding_claim_failed_rs() -> anyhow::Result<()> {
    world().run("scenarios/crowdfunding-claim-failed.scen.json")?;

    Ok(())
}

#[test]
fn crowdfunding_claim_successful_rs() -> anyhow::Result<()> {
    world().run("scenarios/crowdfunding-claim-successful.scen.json")?;

    Ok(())
}

#[test]
fn crowdfunding_claim_too_early_rs() -> anyhow::Result<()> {
    world().run("scenarios/crowdfunding-claim-too-early.scen.json")?;

    Ok(())
}

#[test]
fn crowdfunding_fund_rs() -> anyhow::Result<()> {
    world().run("scenarios/crowdfunding-fund.scen.json")?;

    Ok(())
}

#[test]
fn crowdfunding_fund_too_late_rs() -> anyhow::Result<()> {
    world().run("scenarios/crowdfunding-fund-too-late.scen.json")?;

    Ok(())
}

#[test]
fn crowdfunding_init_rs() -> anyhow::Result<()> {
    world().run("scenarios/crowdfunding-init.scen.json")?;

    Ok(())
}

#[test]
fn egld_crowdfunding_claim_failed_rs() -> anyhow::Result<()> {
    world().run("scenarios/egld-crowdfunding-claim-failed.scen.json")?;

    Ok(())
}

#[test]
fn egld_crowdfunding_claim_successful_rs() -> anyhow::Result<()> {
    world().run("scenarios/egld-crowdfunding-claim-successful.scen.json")?;

    Ok(())
}

#[test]
fn egld_crowdfunding_claim_too_early_rs() -> anyhow::Result<()> {
    world().run("scenarios/egld-crowdfunding-claim-too-early.scen.json")?;

    Ok(())
}

#[test]
fn egld_crowdfunding_fund_rs() -> anyhow::Result<()> {
    world().run("scenarios/egld-crowdfunding-fund.scen.json")?;

    Ok(())
}

#[test]
fn egld_crowdfunding_fund_too_late_rs() -> anyhow::Result<()> {
    world().run("scenarios/egld-crowdfunding-fund-too-late.scen.json")?;

    Ok(())
}

#[test]
fn egld_crowdfunding_init_rs() -> anyhow::Result<()> {
    world().run("scenarios/egld-crowdfunding-init.scen.json")?;

    Ok(())
}
