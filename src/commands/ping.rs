use crate::state::State;
use std::time::Instant;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

/// Replies with Pong! and response time
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {

    let start = Instant::now();

    let elapsed = start.elapsed().as_millis();

    ctx.say(format!(
        "🏓 Pong!\n ⏳ Response Time: {}ms", elapsed
    ))
    .await?;

    Ok(())
}