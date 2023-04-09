use azalea::Account;
use chrono::Local;
use simplelog::*;

use std::fs::OpenOptions;
use std::process::exit;

#[macro_use]
extern crate log;

mod discord;
mod minecraft;

#[tokio::main]
async fn main() {
    init_logger();

    let discord_token = match std::env::var("DISCORD_TOKEN") {
        Ok(v) => v,
        Err(_) => {
            error!("Could not grab token... exiting.");
            exit(1)
        }
    };
    let accounts = vec![Account::offline("bot1")];
    // Run services simultaneously
    let (minecraft, discord) = tokio::join!(
        minecraft::create_swarm(accounts),
        discord::create_client(discord_token)
    );

    match minecraft {
        Ok(_) => (),
        Err(err) => {
            error!("Swarm failed: {}", err);
            exit(2);
        },
    }
    match discord {
        Ok(_) => (),
        Err(err) => {
            error!("Discord failed: {}", err );
            exit(3);
        },
    }
}

fn init_logger() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Error,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Error,
            Config::default(),
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(format!("log-{}.txt", Local::now().format("%Y-%m-%d")))
                .unwrap(),
        ),
    ])
    .unwrap();
}
