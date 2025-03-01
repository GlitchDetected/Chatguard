use crate::state::State;
use poise::serenity_prelude::Colour;
use reqwest;
use serde::Deserialize;
use std::error::Error;

type Context<'a> = poise::Context<'a, State, Box<dyn Error + Send + Sync>>;

#[derive(Deserialize)]
struct ISSResponse {
    message: String,
    timestamp: u64,
    iss_position: ISSPosition,
}

#[derive(Deserialize)]
struct ISSPosition {
    latitude: String,
    longitude: String,
}

/// Fetches and displays the current location of the ISS
#[poise::command(slash_command, prefix_command)]
pub async fn issinfo(ctx: Context<'_>) -> Result<(), Box<dyn Error + Send + Sync>> {
    ctx.defer().await?;

    let response = reqwest::get("http://api.open-notify.org/iss-now.json").await?.json::<ISSResponse>().await?;

    if response.message == "success" {
        let latitude = response.iss_position.latitude;
        let longitude = response.iss_position.longitude;
        let timestamp = response.timestamp;

        let description = format!(
            "The International Space Station (ISS) is currently at:\n- **Latitude:** {}\n- **Longitude:** {} \n- **Timestamp:** {}",
            latitude, longitude, timestamp
        );

        ctx.send(|m| {
            m.embed(|e| {
                e.title("ISS Current Location")
                    .description(description)
                    .colour(Colour::BLUE)
            })
        }).await?;
    } else {
        ctx.say("Unable to fetch the ISS location at the moment.").await?;
    }

    Ok(())
}
