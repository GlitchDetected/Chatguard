use crate::state::State;
use poise::serenity_prelude::Colour;
use std::error::Error;

type Context<'a> = poise::Context<'a, State, Box<dyn Error + Send + Sync>>;

/// Fetches and displays information about a user
#[poise::command(slash_command, prefix_command)]
pub async fn userinfo(
    ctx: Context<'_>,
    #[description = "The user to fetch information about"] user: Option<poise::serenity_prelude::User>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user = user.unwrap_or_else(|| ctx.author().clone());
    let member = ctx.guild_id().and_then(|guild_id| guild_id.to_guild_cached(ctx.serenity_context())).and_then(|g| g.members.get(&user.id).cloned());
    
    let joined_at = member.as_ref().and_then(|m| m.joined_at.map(|d| d.format("%Y-%m-%d").to_string())).unwrap_or("Unknown".to_string());
    let created_at = user.created_at().format("%Y-%m-%d").to_string();
    let roles = member.map(|m| m.roles.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(", ")).unwrap_or("None".to_string());

    ctx.send(|m| {
        m.embed(|e| {
            e.title(format!("{}'s Information", user.name))
                .colour(Colour::BLUE)
                .thumbnail(user.face())
                .field("User Name", &user.name, true)
                .field("User ID", &user.id.to_string(), true)
                .field("Joined Server On", &joined_at, true)
                .field("Account Created On", &created_at, true)
                .field("Roles", &roles, false)
        })
    }).await?;
    
    Ok(())
}