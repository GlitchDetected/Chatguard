use crate::state::State;
use poise::serenity_prelude::Colour;
use std::{error::Error, time::SystemTime};

type Context<'a> = poise::Context<'a, State, Box<dyn Error + Send + Sync>>;

/// Displays the bot's uptime
#[poise::command(slash_command, prefix_command)]
pub async fn uptime(ctx: Context<'_>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let start_time = ctx.data().start_time;
    let uptime = SystemTime::now()
    .duration_since(start_time)
    .unwrap_or_default();

    let seconds = uptime.as_secs();
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    ctx.send(|m| {
        m.embed(|e| {
            e.title("⏳ Bot Uptime")
                .description(format!(
                    "🕑 **{} days, {} hours, {} minutes, and {} seconds**",
                    days, hours, minutes, seconds
                ))
                .colour(Colour::from_rgb(66, 133, 244))
        })
    })
    .await?;

    Ok(())
}
