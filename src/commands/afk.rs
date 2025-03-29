use crate::state::State;
use crate::models::{Afk, NewAfk};
use crate::schema::afkusers::dsl::*;
use diesel::prelude::*;
use chrono::Utc;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

/// Sets your AFK status with a required message.
#[poise::command(slash_command, prefix_command)]
pub async fn afk(ctx: Context<'_>, user_message: String) -> Result<(), Error> {

    let user_id_str = ctx.author().id.to_string();
    let now = Utc::now().naive_utc();

    // Check if the user is already AFK
    let existing_afk = afkusers
        .filter(user_id.eq(&user_id_str))
        .first::<Afk>(conn)
        .optional()?;

    if let Some(entry) = existing_afk {
        let afk_duration = (now - entry.created_at).num_seconds();
        let duration_string = format_duration(afk_duration);

        ctx.say(format!(
            "You are already AFK for {}. Use `/back` or return to reset your AFK status.",
            duration_string
        ))
        .await?;
        return Ok(());
    }

    // Insert new AFK entry
    let new_afk = NewAfk {
        user_id: &user_id_str,
        message: &user_message,
        duration: 0,
        created_at: now,
    };

    diesel::insert_into(afkusers)
        .values(&new_afk)
        .execute(conn)?;

    ctx.say(format!("You are now AFK. Message: \"{}\"", user_message))
        .await?;

    Ok(())
}

/// Formats duration into a human-readable string
fn format_duration(seconds: i64) -> String {
    let years = seconds / 31_536_000;
    let days = (seconds % 31_536_000) / 86_400;
    let hours = (seconds % 86_400) / 3_600;
    let minutes = (seconds % 3_600) / 60;
    let secs = seconds % 60;

    let mut parts = Vec::new();
    if years > 0 { parts.push(format!("{}y", years)); }
    if days > 0 { parts.push(format!("{}d", days)); }
    if hours > 0 { parts.push(format!("{}h", hours)); }
    if minutes > 0 { parts.push(format!("{}m", minutes)); }
    if secs > 0 { parts.push(format!("{}s", secs)); }

    parts.join(" ")
}
