use crate::state::State;
use poise::serenity_prelude::{Colour};
use std::error::Error;

type Context<'a> = poise::Context<'a, State, Box<dyn Error + Send + Sync>>;

/// Provides a link to vote for ChatGuard
#[poise::command(slash_command, prefix_command)]
pub async fn vote(ctx: Context<'_>) -> Result<(), Box<dyn Error + Send + Sync>> {
    ctx.send(|m| {
        m.embed(|e| {
            e.title("🗳️ Vote for ChatGuard!")
                .description("Please vote for ChatGuard on top.gg - [Vote Now](https://top.gg/bot/1237878380838523001#vote)")
                .colour(Colour::from_rgb(66, 133, 244))
        })
        .components(|c| {
            c.create_action_row(|r| {
                r.create_button(|b| {
                    b.label("Vote Now")
                        .style(poise::serenity_prelude::ButtonStyle::Link)
                        .url("https://top.gg/bot/1237878380838523001#vote")
                })
            })
        })
    }).await?;

    Ok(())
}
