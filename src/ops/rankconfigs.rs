use crate::args::{
    RankConfigsCommand, 
    RankConfigsSubcommand, 
    ConfigureRank
};

use crate::db::establish_connection;
use crate::models::{NewRankConfig, RankConfig};
use diesel::prelude::*;

pub fn handle_rankconfigs_command(rankconfigs: RankConfigsCommand) {
    let command = rankconfigs.command;
    match command {
        RankConfigsSubcommand::Configure(rankconfig) => {
            configure_rank(rankconfig);
        }
        RankConfigsSubcommand::Show => {
            show_rankconfigs();
        }
    }
}

fn configure_rank(rankconfig: ConfigureRank) {
    println!("Configuring rank settings for guild: {}", rankconfig.guild_id);
    use crate::schema::rankconfigs::dsl::*;

    let mut connection = establish_connection();
    let new_rankconfig = NewRankConfig {
        guild_id: &rankconfig.guild_id,
        rankchannel: rankconfig.rankchannel.as_deref(),
        rankconfigure: rankconfig.rankconfigure,
    };

    diesel::insert_into(rankconfigs)
        .values(&new_rankconfig)
        .on_conflict(guild_id)
        .do_update()
        .set((
            rankchannel.eq(rankconfig.rankchannel.clone().as_deref()),
            rankconfigure.eq(rankconfig.rankconfigure),
        ))
        .execute(&mut connection)
        .expect("Error configuring rank settings");
}

fn show_rankconfigs() {
    use crate::schema::rankconfigs::dsl::*;

    let mut connection = establish_connection();
    let results = rankconfigs
        .load::<RankConfig>(&mut connection)
        .expect("Error loading rank configurations");

    println!("Displaying {} rank configurations", results.len());
    for config in results {
        println!(
            "Guild ID: {}, Rank Channel: {:?}, Configured: {}",
            config.guild_id, 
            config.rankchannel, 
            config.rankconfigure
        );
    }
}