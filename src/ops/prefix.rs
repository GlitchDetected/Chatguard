use crate::args::{
    PrefixCommand, 
    PrefixSubcommand, 
    CreatePrefix,
    UpdatePrefix,
    DeleteEntity,
};

use crate::db::establish_connection;
use crate::models::{NewPrefix, Prefix};
use diesel::prelude::*;

pub fn handle_prefix_command(prefix: PrefixCommand) {
    let command = prefix.command;
    match command {
        PrefixSubcommand::Create(prefix) => {
            create_prefix(prefix);
        }
        PrefixSubcommand::Update(prefix) => {
            update_prefix(prefix);
        }
        PrefixSubcommand::Delete(delete_entity) => {
            delete_prefix(delete_entity);
        }
        PrefixSubcommand::Show => {
            show_prefixes();
        }
    }
}

fn create_prefix(prefix: CreatePrefix) {
    println!("Setting prefix for guild: {}", prefix.guild_id);
    use crate::schema::prefixes::dsl::*;

    let mut connection = establish_connection();
    let new_prefix = NewPrefix {
        guild_id: &prefix.guild_id,
        guildprefix: &prefix.guildprefix,
    };

    diesel::insert_into(prefixes)
    .values(&new_prefix)
    .execute(&mut connection)
    .expect("Error setting prefix");
}

fn update_prefix(prefix: UpdatePrefix) {
    println!("Updating prefix for guild: {}", prefix.guild_id);
    use crate::schema::prefixes::dsl::*;

    let mut connection = establish_connection();
    diesel::update(prefixes.filter(guild_id.eq(&prefix.guild_id)))
        .set(guildprefix.eq(&prefix.guildprefix))
        .execute(&mut connection)
        .expect("Error updating prefix");
}

fn delete_prefix(prefix: DeleteEntity) {
    println!("Removing prefix for guild ID: {}", prefix.id);
    use crate::schema::prefixes::dsl::*;

    let mut connection = establish_connection();
    diesel::delete(prefixes.filter(guild_id.eq(prefix.id.to_string())))
        .execute(&mut connection)
        .expect("Error removing autorole");
}

fn show_prefixes() {
    use crate::schema::prefixes::dsl::*;

    let mut connection = establish_connection();
    let results = prefixes
        .load::<Prefix>(&mut connection)
        .expect("Error loading prefixes");

    println!("Displaying {} prefixes", results.len());
    for prefix in results {
        println!("{:?}", prefix);
    }
}
