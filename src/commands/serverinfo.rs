use crate::state::State;
use poise::serenity_prelude::Colour;
use std::error::Error;

type Context<'a> = poise::Context<'a, State, Box<dyn Error + Send + Sync>>;

/// Fetches and displays information about the current server
#[poise::command(slash_command, prefix_command)]
pub async fn serverinfo(ctx: Context<'_>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let guild = match ctx.guild() {
        Some(g) => g,
        None => {
            ctx.say("This command can only be used in a server.").await?;
            return Ok(());
        }
    };

    let owner_id = guild.owner_id;
    let member_count = guild.member_count;
    let created_at = guild.id.created_at().format("%Y-%m-%d").to_string();
    let region = guild.preferred_locale.clone();
    let icon_url = guild.icon_url();
    let verification_level = format!("{:?}", guild.verification_level);

    let owner = match ctx.http().get_user(owner_id.0).await {
        Ok(user) => user.name,
        Err(_) => "Unknown Owner".to_string(),
    };

    ctx.send(|m| {
        m.embed(|e| {
            e.title(&guild.name)
                .colour(Colour::DARK_GREEN)
                .thumbnail(icon_url.unwrap_or_default())
                .field("Server Name", &guild.name, true)
                .field("Server ID", &guild.id.to_string(), true)
                .field("Owner", &owner, true)
                .field("Member Count", &member_count.to_string(), true)
                .field("Created At", &created_at, true)
                .field("Region", &region, true)
                .field("Verification Level", &verification_level, true)
        })
    }).await?;

    Ok(())
}
