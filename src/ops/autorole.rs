use crate::args::{
    AutoroleCommand, 
    AutoroleSubcommand, 
    CreateAutorole,
    UpdateAutorole,
    DeleteEntity
};

use crate::db::establish_connection;
use crate::models::{NewAutoRole, AutoRole};
use diesel::prelude::*;

pub fn handle_autorole_command(autorole: AutoroleCommand) {
    let command = autorole.command;
    match command {
        AutoroleSubcommand::Create(autorole) => {
            create_autorole(autorole);
        }
        AutoroleSubcommand::Update(autorole) => {
            update_autorole(autorole);
        }
        AutoroleSubcommand::Delete(delete_entity) => {
            remove_autorole(delete_entity);
        }
        AutoroleSubcommand::Show => {
            show_autoroles();
        }
    }
}

fn create_autorole(autorole: CreateAutorole) {
    println!("Creating autorole for guild: {}", &autorole.guild_id);
    use crate::schema::autoroles::dsl::*;

    let mut connection = establish_connection();
    let new_autorole = NewAutoRole {
        guild_id: &autorole.guild_id,
        role_id: &autorole.role_id,
        created_at: autorole.created_at,
        updated_at: autorole.updated_at
    };

    diesel::insert_into(autoroles)
        .values(&new_autorole)
        .execute(&mut connection)
        .expect("Error creating autorole");
}

fn update_autorole(autorole: UpdateAutorole) {
    println!("Updating autorole for guild: {}", autorole.guild_id);
    use crate::schema::autoroles::dsl::*;

    let mut connection = establish_connection();
    diesel::update(autoroles.filter(guild_id.eq(&autorole.guild_id)))
        .set(role_id.eq(&autorole.role_id))
        .execute(&mut connection)
        .expect("Error updating autorole");
}

fn remove_autorole(autorole: DeleteEntity) {
    println!("Removing autorole for guild ID: {}", autorole.id);
    use crate::schema::autoroles::dsl::*;

    let mut connection = establish_connection();
    diesel::delete(autoroles.filter(guild_id.eq(autorole.id.to_string())))
        .execute(&mut connection)
        .expect("Error removing autorole");
}

fn show_autoroles() {
    use crate::schema::autoroles::dsl::*;

    let mut connection = establish_connection();
    let results = autoroles
        .load::<AutoRole>(&mut connection)
        .expect("Error loading autoroles");

    println!("Displaying {} autoroles", results.len());
    for role in results {
        println!("{:?}", role);
    }
}
