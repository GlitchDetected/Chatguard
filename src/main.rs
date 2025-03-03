#[macro_use]
extern crate diesel;

mod commands;
mod events;
mod state;

mod args;
mod ops;
mod db;
mod schema;
mod models;

use ops::afk::handle_afk_command;
use ops::autorole::handle_autorole_command;
use ops::commands::handle_commands_command;
use ops::prefix::handle_prefix_command;
use ops::rank::handle_rank_command;
use ops::rankconfigs::handle_rankconfigs_command;
use ops::ticketconfigs::handle_ticketconfigs_command;
use ops::tickets::handle_tickets_command;
use ops::uptime::handle_uptime_command;

use args::EntityType;
use args::ChatguardArgs;
use clap::Parser;

use dotenv::dotenv;
use tracing_subscriber::fmt::init;
use events::main::ready;

#[tokio::main]
async fn main() {
    let args = ChatguardArgs::parse();

    match args.entity_type {
        EntityType::Afk(afk) => handle_afk_command(afk),
        EntityType::Autorole(autorole) => handle_autorole_command(autorole),
        EntityType::Commands(commands) => handle_commands_command(commands),
        EntityType::Prefix(prefix) => handle_prefix_command(prefix),
        EntityType::Rank(rank) => handle_rank_command(rank),
        EntityType::RankConfigs(rank_configs) => handle_rankconfigs_command(rank_configs),
        EntityType::TicketConfigs(ticket_configs) => handle_ticketconfigs_command(ticket_configs),
        EntityType::Tickets(tickets) => handle_tickets_command(tickets),
        EntityType::Uptime(uptime) => handle_uptime_command(uptime),
    };

    dotenv().ok();
    init();
    ready().await;
}