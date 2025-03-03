use crate::args::{
    AfkSubcommand, 
    AfkCommand, 
    CreateAfk,
    DeleteEntity,
};
use crate::db::establish_connection;
use crate::models::{NewAfk, Afk};
use diesel::prelude::*;

pub fn handle_afk_command(afk: AfkCommand) {
    let command = afk.command;
    match command {
        AfkSubcommand::Create(afk) => {
            create_afk(afk);
        }
        AfkSubcommand::Delete(delete_entity) => {
            remove_afk(delete_entity);
        }
        AfkSubcommand::Show => {
            show_afk_users();
        }
    }
}

pub fn create_afk(afk: CreateAfk) {
    println!("Setting AFK status for user: {}", &afk.user_id);
    use crate::schema::afkusers::dsl::*;

    let mut connection = establish_connection();
    let new_afk = NewAfk {
        user_id: &afk.user_id,
        message: &afk.message,
        duration: afk.duration,
        created_at: afk.created_at,
    };

    diesel::insert_into(afkusers)
        .values(&new_afk)
        .execute(&mut connection)
        .expect("Error setting AFK status");
}

fn remove_afk(afk: DeleteEntity) {
    println!("Removing AFK status for {:?}", afk);
    use crate::schema::afkusers::dsl::*;

    let mut connection = establish_connection();
    diesel::delete(afkusers.filter(user_id.eq(afk.id.to_string())))
        .execute(&mut connection)
        .expect("Error removing AFK status");
}

fn show_afk_users() {
    println!("Showing afk users");
    use crate::schema::afkusers::dsl::*;

    let mut connection = establish_connection();
    let results = afkusers
    .load::<Afk>(&mut connection)
    .expect("error showing afk users");

    println!("Displaying {} AFK users", results.len());
    for afk in results {
        println!("{:?}", afk);
    }
}
