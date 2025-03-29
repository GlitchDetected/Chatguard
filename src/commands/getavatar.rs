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
        "getavatar",
        "Get the avatar of a user",
        CommandType::ChatInput,
    )
    .option(
        StringBuilder::new("user", "the user you want the avatar for")
            .required(true)
    )
    .build()]
}

pub async fn getavatar(
    ctx: &Context,
    interaction: &Interaction,
    command: &CommandData,
    user: String,
) -> Result<(), Error> {
        let userid = command.options.iter().find_map(|opt| match &opt.value {
            CommandOptionValue::User(user_id) => Some(*user_id),
            _ => None,
    });

    let user_id = match userid {
        Some(id) => id,
        None => {
            return Err(Error::Custom("No user provided!".to_string()));
        }
    };

    let user = ctx.http_client().user(user_id).await?.model().await?;
    
    // Better way to get the avatar URL
    let avatar_url = user.avatar_url().unwrap_or_else(|| user.default_avatar_url());

    let data = EmbedBuilder::new()
        .title(&user.name)
        .image(avatar_url)
        .into_ephemeral();

    create_response(ctx, interaction, data).await?;

    Ok(())
}
