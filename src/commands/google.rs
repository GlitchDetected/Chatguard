use urlencoding::encode;

use twilight_model::application::command::{Command, CommandType};
use twilight_model::application::interaction::application_command::{
    CommandData, CommandOptionValue,
};
use twilight_model::application::interaction::Interaction;
use twilight_util::builder::command::{CommandBuilder, StringBuilder};
use twilight_util::builder::embed::EmbedBuilder;

use super::create_response;
use crate::bot::Context;
use crate::commands::EphemeralMessage;
use crate::error::Error;

pub fn commands() -> Vec<Command> {
    vec![CommandBuilder::new(
        "google",
        "Google something special",
        CommandType::ChatInput,
    )
    .option(
        StringBuilder::new("command", "googler")
            .required(true)
    )
    .build()]
}

pub async fn google(
    ctx: &Context,
    interaction: &Interaction,
    command: &CommandData,
    query: String,
) -> Result<(), Error> {
    let command = command.options.iter().find_map(|opt| match &opt.value {
        CommandOptionValue::String(value) => Some(value.as_str()),
        _ => None,
    });
    
    let encoded_query = encode(&query);
    let google_url = format!("https://www.google.com/search?q={}", encoded_query);

    let data = EmbedBuilder::new()
    .title("Google Search")
    .description(format!("Here is your [Google search result]({}) for: **{}**", google_url, query))
    .into_ephemeral();

    create_response(ctx, interaction, data).await?;

    Ok(())
}
