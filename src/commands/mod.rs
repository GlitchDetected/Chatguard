use twilight_http::client::InteractionClient;
use twilight_model::application::command::Command;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::message::MessageFlags;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};
use twilight_util::builder::embed::EmbedBuilder;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::bot::Context;
use crate::error::Error;

mod afk;
mod getavatar;
mod google;
mod help;
mod issinfo;
mod ping;
mod serverinfo;
mod uptime;
mod userinfo;
mod vote;

fn commands() -> Vec<Command> {
    let mut cmds = Vec::new();
    cmds.extend(afk::commands());
    cmds.extend(getavatar::commands());
    cmds.extend(google::commands());
    cmds.extend(help::commands());
    cmds.extend(issinfo::commands());
    cmds.extend(ping::commands());
    cmds.extend(serverinfo::commands());
    cmds.extend(uptime::commands());
    cmds.extend(userinfo::commands());
    cmds.extend(vote::commands());
    cmds
}

pub async fn register(client: &InteractionClient<'_>) -> Result<(), Error> {
    client.set_global_commands(&commands()).await?;
    Ok(())
}

pub async fn handle_command(ctx: &Context, interaction: &Interaction, command: &CommandData) {
    let res = match command.name.as_str() {
        "afk" => afk::afk(ctx, interaction, command).await,
        "getavatar" => getavatar::getavatar(ctx, interaction, command).await,
        "google" => google::google(ctx, interaction, command).await,
        "help" => help::help(ctx, interaction, command).await,
        "issinfo" => issinfo::issinfo(ctx, interaction, command).await,
        "ping" => ping::ping(ctx, interaction, command).await,
        "serverinfo" => serverinfo::serverinfo(ctx, interaction, command).await,
        "uptime" => uptime::uptime(ctx, interaction, command).await,
        "userinfo" => userinfo::userinfo(ctx, interaction, command).await,
        "vote" => vote::vote(ctx, interaction, command).await,
        _ => Ok(()),
    };
    if let Err(e) = res {
        tracing::error!("{e}");
    }
}

pub async fn handle_component(
    ctx: &Context,
    interaction: &Interaction,
    component: &MessageComponentInteractionData,
) {
    let res = match interaction
        .message
        .as_ref()
        .and_then(|m| m.interaction.as_ref())
    {
        Some(msg) if msg.name == "mods" => mods::list_component(ctx, interaction, component).await,
        _ => Ok(()),
    };
    if let Err(e) = res {
        tracing::error!("{e}");
    }
}

trait EphemeralMessage {
    fn into_ephemeral(self) -> InteractionResponseData;
}

impl EphemeralMessage for &str {
    fn into_ephemeral(self) -> InteractionResponseData {
        InteractionResponseDataBuilder::new()
            .ephemeral(self)
            .build()
    }
}

impl EphemeralMessage for String {
    fn into_ephemeral(self) -> InteractionResponseData {
        InteractionResponseDataBuilder::new()
            .ephemeral(self)
            .build()
    }
}

impl EphemeralMessage for EmbedBuilder {
    fn into_ephemeral(self) -> InteractionResponseData {
        let embed = self.build();
        InteractionResponseDataBuilder::new()
            .flags(MessageFlags::EPHEMERAL)
            .embeds([embed])
            .build()
    }
}

trait InteractionResponseBuilderExt {
    fn ephemeral(self, content: impl Into<String>) -> InteractionResponseDataBuilder;
}

impl InteractionResponseBuilderExt for InteractionResponseDataBuilder {
    fn ephemeral(self, content: impl Into<String>) -> InteractionResponseDataBuilder {
        self.content(content).flags(MessageFlags::EPHEMERAL)
    }
}

async fn create_response(
    ctx: &Context,
    interaction: &Interaction,
    data: InteractionResponseData,
) -> Result<(), Error> {
    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(data),
    };
    ctx.interaction()
        .create_response(interaction.id, &interaction.token, &response)
        .await?;
    Ok(())
}

async fn defer_ephemeral(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    ctx.interaction()
        .create_response(
            interaction.id,
            &interaction.token,
            &InteractionResponse {
                kind: InteractionResponseType::DeferredChannelMessageWithSource,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .flags(MessageFlags::EPHEMERAL)
                        .build(),
                ),
            },
        )
        .await?;
    Ok(())
}

async fn defer_response(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    ctx.interaction()
        .create_response(
            interaction.id,
            &interaction.token,
            &InteractionResponse {
                kind: InteractionResponseType::DeferredChannelMessageWithSource,
                data: None,
            },
        )
        .await?;
    Ok(())
}

async fn defer_component_response(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    ctx.interaction()
        .create_response(
            interaction.id,
            &interaction.token,
            &InteractionResponse {
                kind: InteractionResponseType::DeferredUpdateMessage,
                data: None,
            },
        )
        .await?;
    Ok(())
}

async fn update_response_content(
    ctx: &Context,
    interaction: &Interaction,
    content: &str,
) -> Result<(), Error> {
    ctx.interaction()
        .update_response(&interaction.token)
        .content(Some(content))?
        .await?;
    Ok(())
}

async fn update_response_from_content(
    ctx: &Context,
    interaction: &Interaction,
    title: &str,
    contents: &[String],
) -> Result<(), Error> {
    let mut contents = contents.iter();
    if let Some(content) = contents.next() {
        let embed = EmbedBuilder::new()
            .title(title)
            .description(content)
            .build();

        ctx.interaction()
            .update_response(&interaction.token)
            .embeds(Some(&[embed]))?
            .await?;

        for content in contents {
            let embed = EmbedBuilder::new()
                .title(title)
                .description(content)
                .build();

            ctx.interaction()
                .create_followup(&interaction.token)
                .embeds(&[embed])?
                .await?;
        }
    }
    Ok(())
}