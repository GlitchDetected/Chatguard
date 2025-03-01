use crate::state::State;
use poise::serenity_prelude::{Colour, User};
use std::error::Error;

type Context<'a> = poise::Context<'a, State, Box<dyn Error + Send + Sync>>;

/// Retrieves a user's avatar
#[poise::command(slash_command, prefix_command)]
pub async fn getavatar(
    ctx: Context<'_>,
    #[description = "The user whose avatar you want to see"] user: Option<User>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user = user.unwrap_or_else(|| ctx.author().clone());
    let avatar_url = user.avatar_url().unwrap_or_else(|| user.default_avatar_url());

    ctx.send(|m| {
        m.embed(|e| {
            e.title(&user.name)
                .image(avatar_url)
                .colour(Colour::from_rgb(150, 75, 0)) // Brown color
        })
    })
    .await?;

    Ok(())
}
