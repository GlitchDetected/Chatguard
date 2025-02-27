mod commands;
mod events;
mod state;

use dotenv::dotenv;
use tracing_subscriber::fmt::init;
use events::main::ready;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init();
    ready().await;
}