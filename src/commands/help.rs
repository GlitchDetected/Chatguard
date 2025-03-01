use crate::state::State;
use poise::serenity_prelude::Colour;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

/// Help command!
#[poise::command(slash_command, prefix_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {

    ctx.send(|m| {
        m.embed(|e| {
            e.title(format!("This is the help command!"))
                .colour(Colour::BLUE)
                .description("fetch information about the bot")
        })
    }).await?;
    
    Ok(())
}