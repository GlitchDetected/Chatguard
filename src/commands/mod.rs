mod ping;

use poise::Command;

pub fn getallcmd() -> Vec<Command<crate::state::State, Box<dyn std::error::Error + Send + Sync>>> {
    vec![
        ping::ping(),
    ]
}