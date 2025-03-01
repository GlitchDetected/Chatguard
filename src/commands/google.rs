use crate::state::State;
use poise::serenity_prelude::Colour;
use std::error::Error;
use urlencoding::encode;

type Context<'a> = poise::Context<'a, State, Box<dyn Error + Send + Sync>>;

/// Performs a Google search and returns a link.
#[poise::command(slash_command, prefix_command)]
pub async fn google(
    ctx: Context<'_>,
    #[description = "The query to search for"] query: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    
    let encoded_query = encode(&query);
    let google_url = format!("https://www.google.com/search?q={}", encoded_query);

    ctx.send(|m| {
        m.embed(|e| {
            e.title("Google Search")
                .description(format!(
                    "Here is your [Google search result]({}) for: **{}**",
                    google_url, query
                ))
                .colour(Colour::from_rgb(66, 133, 244)) // Google's blue color
                .footer(|f| f.text("Google Search | Powered by ChatGuard"))
        })
    })
    .await?;

    Ok(())
}
