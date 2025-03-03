use crate::args::{
    UptimeCommand, 
    UptimeSubcommand, 
    CreateUptime,
    UpdateUptime,
    DeleteEntity
};

use crate::db::establish_connection;
use crate::models::{NewUptime, Uptime};
use diesel::prelude::*;

pub fn handle_uptime_command(uptime: UptimeCommand) {
    let command = uptime.command;
    match command {
        UptimeSubcommand::Create(uptime) => {
            create_uptime(uptime);
        }
        UptimeSubcommand::Update(uptime) => {
            update_uptime(uptime);
        }
        UptimeSubcommand::Delete(delete_entity) => {
            remove_uptime(delete_entity);
        }
        UptimeSubcommand::Show => {
            show_uptime();
        }
    }
}

fn create_uptime(uptime: CreateUptime) {
    println!("Creating uptime: {}", uptime.botuptime);
    use crate::schema::uptimes::dsl::*;

    let mut connection = establish_connection();
    let new_uptime = NewUptime {
        id: uptime.id,
        botuptime: uptime.botuptime,
        created_at: uptime.created_at,
        updated_at: uptime.updated_at,
    };

    diesel::insert_into(uptimes)
        .values(&new_uptime)
        .execute(&mut connection)
        .expect("Error creating command");
}

fn update_uptime(uptime: UpdateUptime) {
    println!("Updating uptime: {}", uptime.botuptime);
    use crate::schema::uptimes::dsl::*;

    let mut connection = establish_connection();
    diesel::update(uptimes.filter(id.eq(&uptime.id)))
        .set(botuptime.eq(&uptime.botuptime))
        .execute(&mut connection)
        .expect("Error updating autorole");
}

fn remove_uptime(uptime: DeleteEntity) {
    println!("Removing uptime: {:?}", uptime);
    use crate::schema::uptimes::dsl::*;

    let mut connection = establish_connection();
    diesel::delete(uptimes.find(uptime.id))
        .execute(&mut connection)
        .expect("Error removing uptime");
}

fn show_uptime() {
    use crate::schema::uptimes::dsl::*;

    let mut connection = establish_connection();
    let results = uptimes
        .load::<Uptime>(&mut connection)
        .expect("Error loading uptime data");

    if let Some(latest) = results.last() {
        println!("Bot Uptime: {}ms", latest.botuptime);
    } else {
        println!("No uptime data available.");
    }
}
