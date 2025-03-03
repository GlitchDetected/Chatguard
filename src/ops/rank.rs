use crate::args::{
    RankCommand, 
    RankSubcommand, 
    CreateRank,
    UpdateRank,
    DeleteEntity,
};

use crate::db::establish_connection;
use crate::models::{NewRank, Rank};
use diesel::prelude::*;

pub fn handle_rank_command(rank: RankCommand) {
    let command = rank.command;
    match command {
        RankSubcommand::Create(rank) => {
            create_rank(rank);
        }
        RankSubcommand::Update(rank) => {
            update_rank(rank);
        }
        RankSubcommand::Delete(delete_entity) => {
            delete_rank(delete_entity);
        }
        RankSubcommand::Show => {
            show_ranks();
        }
    }
}

fn create_rank(rank: CreateRank) {
    println!("Adding rank: {:?}", rank);
    use crate::schema::ranks::dsl::*; // ranks and rank are being mixed up for some reason...

    let mut connection = establish_connection();
    let new_rank = NewRank {
        user_id: &rank.user_id,
        guild_id: &rank.guild_id,
        xp: rank.xp,
        level: rank.level,
        last_daily: rank.last_daily, 
        bg_color: &rank.bg_color,
        bar_color: &rank.bar_color,
        created_at: rank.created_at,
        updated_at: rank.updated_at
    };

    diesel::insert_into(ranks)
        .values(&new_rank)
        .execute(&mut connection)
        .expect("Error adding Rank to DB");
}

fn update_rank(rank: UpdateRank) {
    println!("Updating rank: {:?}", rank);
    use crate::schema::ranks::dsl::*;

    let mut connection = establish_connection();
    diesel::update(ranks.filter(guild_id.eq(&rank.guild_id)))
        .set(xp.eq(&rank.xp))
        .execute(&mut connection)
        .expect("Error updating autorole");
}

pub fn delete_rank(rank: DeleteEntity) {
    println!("Deleting rank: {:?}", rank);
    use crate::schema::ranks::dsl::*;

    let mut connection = establish_connection();
    diesel::delete(ranks.filter(guild_id.eq(rank.id.to_string())))
        .execute(&mut connection)
        .expect("Error removing autorole");
}

fn show_ranks() {
    println!("Showing ranks");
    use crate::schema::ranks::dsl::*;

    let mut connection = establish_connection();
    let results = ranks
        .load::<Rank>(&mut connection)
        .expect("Error loading ranks");

    println!("Displaying {} ranks", results.len());
    for rank in results {
        println!("{:?}", rank);
    }
}
