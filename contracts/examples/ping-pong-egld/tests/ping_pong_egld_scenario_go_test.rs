use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn ping_pong_call_get_user_addresses_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-get-user-addresses.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_ping_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-ping.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_ping_after_deadline_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-ping-after-deadline.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_ping_before_activation_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-ping-before-activation.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_ping_before_beginning_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-ping-before-beginning.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_ping_second_user_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-ping-second-user.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_ping_twice_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-ping-twice.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_ping_wrong_ammount_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-ping-wrong-ammount.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_pong_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-pong.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_pong_all_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-pong-all.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_pong_all_after_pong_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-pong-all-after-pong.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_pong_all_interrupted_1_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-pong-all-interrupted-1.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_pong_all_interrupted_2_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-pong-all-interrupted-2.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_pong_before_deadline_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-pong-before-deadline.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_pong_twice_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-pong-twice.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_call_pong_without_ping_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-call-pong-without-ping.scen.json")?;

    Ok(())
}

#[test]
fn ping_pong_init_go() -> anyhow::Result<()> {
    world().run("scenarios/ping-pong-init.scen.json")?;

    Ok(())
}
