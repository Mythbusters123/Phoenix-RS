extern crate simplelog;
use azalea::{prelude::*, Swarm, SwarmEvent};
use std::error::Error;
use std::time::Duration;
mod state;
// Thanks to ShayBox
#[derive(Default, Clone, Component)]
struct State;

#[derive(Default, Clone, Resource, Component)]
struct SwarmState {}

// Azalea Plugin
pub async fn create_swarm(accounts: Vec<Account>) -> Result<(), Box<dyn Error>> {
    // temporary
    let e = SwarmBuilder::new()
        .add_accounts(accounts.clone())
        .set_handler(handler)
        .set_swarm_handler(swarm_handler)
        .join_delay(Duration::from_millis(1500))
        .start("hypixel.net")
        .await;
    match e {
        Ok(()) => Ok(()),
        Err(why) => Err(Box::new(why)),
    }
}

#[allow(dead_code)]
async fn handler(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            println!("{} ({}) logged in!", bot.profile.name, bot.profile.uuid);
            // Send to limbo
            bot.send_command_packet("achat §c");
        }
        Event::Chat(m) => {
            // Only act if guild message
            if m.message().to_string().starts_with("Guild>") {}
        }
        _ => {}
    }
    Ok(())
}
#[allow(dead_code)]
async fn swarm_handler(
    mut swarm: Swarm,
    event: SwarmEvent,
    _state: SwarmState,
) -> anyhow::Result<()> {
    match event {
        SwarmEvent::Login => todo!(),
        SwarmEvent::Init => todo!(),
        // Implement auto-reconnect
        SwarmEvent::Disconnect(account) => {
            warn!(
                "Bot got kicked! {} ({})",
                account.username,
                account.uuid.unwrap().to_string()
            );
            tokio::time::sleep(Duration::from_secs(5)).await;
            swarm
                .add_with_exponential_backoff(&account, State::default())
                .await;
        }
        SwarmEvent::Chat(_m) => {
            // Use this as a "Tick" event
        }
    }
    Ok(())
}
