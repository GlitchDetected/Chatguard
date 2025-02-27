use crate::commands::getallcmd;
use crate::state::State;
use poise::FrameworkBuilder;
use std::collections::HashMap;
use tracing::error;

pub fn registercmds() -> FrameworkBuilder<State, Box<dyn std::error::Error + Send + Sync>> {
    let commands = getallcmd();

    let mut prevcmds: HashMap<String, String> = HashMap::new();

    for cmd in &commands {
        let cmd_name = cmd.name.clone();
        let cmd_desc = cmd.description.clone().unwrap_or_else(|| "No description".to_string());

        if let Some(old_desc) = prevcmds.get(&cmd_name) {
            if old_desc != &cmd_desc {
                println!("📝 Edited Command: {} (Updated description)", cmd_name);
            }
        } else {
            println!("🚀 Registered Command: {}", cmd_name);
        }

        prevcmds.insert(cmd_name, cmd_desc);
    }

    for oldcmd in prevcmds.keys() {
        if !commands.iter().any(|cmd| &cmd.name == oldcmd) {
            println!("❌ Deleted Command: {}", oldcmd);
        }
    }

    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(".".into()),
                ..Default::default()
            },
            on_error: |error| Box::pin(async move { error!("{error}") }),
            ..Default::default()
        })
}
