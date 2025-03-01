use crate::events::commands::registercmds;
use crate::state::State;
use poise::serenity_prelude::{self as serenity};
use std::env;
use serenity::model::gateway::Activity;
use serenity::model::user::OnlineStatus;

pub async fn ready() {
    let framework = registercmds()
        .token(env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::all())
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                let state = State::default();

                let activity = Activity::playing("Give ChatGuard a vote! | /help");
                let status = OnlineStatus::Idle;
                ctx.set_presence(Some(activity.clone()), status).await;

                let statuslogger = match status {
                    OnlineStatus::Online => "🌿",
                    OnlineStatus::Idle => "🌙",
                    OnlineStatus::DoNotDisturb => "🔴",
                    OnlineStatus::Invisible | OnlineStatus::Offline => "⚫",
                    _ => "❓",
                };

                println!(
                    "{} is now {} ({})",
                    ready.user.tag(),
                    statuslogger,
                    activity.name
                );

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let sm = framework.shard_manager().clone();
                tokio::spawn(async move {
                    tokio::signal::ctrl_c()
                        .await
                        .expect("failed to listen for ctrl+c");
                    sm.lock().await.shutdown_all().await;
                });
                Ok(state)
            })
        });

    framework.run_autosharded().await.unwrap();
}
