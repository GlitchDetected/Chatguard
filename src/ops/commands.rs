use crate::args::{
    CommandsCommand, 
    CommandsSubcommand, 
    CreateCommand,
    UpdateCommand,
    DeleteEntity
};

use crate::db::establish_connection;
use crate::models::{NewCommand, Command};
use diesel::prelude::*;

pub fn handle_commands_command(commands: CommandsCommand) {
    let command = commands.command;
    match command {
        CommandsSubcommand::Create(command) => {
            create_command(command);
        }
        CommandsSubcommand::Update(command) => {
            update_command(command);
        }
        CommandsSubcommand::Delete(delete_entity) => {
            remove_command(delete_entity);
        }
        CommandsSubcommand::Show => {
            show_commands();
        }
    }
}

fn create_command(command: CreateCommand) {
    println!("Creating command: {}", command.name);
    use crate::schema::commands::dsl::*;

    let mut connection = establish_connection();
    let new_command = NewCommand {
        id: &command.id,
        name: &command.name,
        description: &command.description,
    };

    diesel::insert_into(commands)
        .values(&new_command)
        .execute(&mut connection)
        .expect("Error creating command");
}

fn update_command(command: UpdateCommand) {
    println!("Updating command: {}", command.id);
    use crate::schema::commands::dsl::*;

    let mut connection = establish_connection();
    diesel::update(commands.filter(id.eq(&command.id)))
        .set((
            name.eq(&command.name), 
            description.eq(&command.description)
        ))
        .execute(&mut connection)
        .expect("Error updating command");
}

fn remove_command(command: DeleteEntity) {
    println!("Removing command with ID: {}", command.id);
    use crate::schema::commands::dsl::*;

    let mut connection = establish_connection();
    diesel::delete(commands.filter(id.eq(command.id.to_string())))
        .execute(&mut connection)
        .expect("Error removing command");
}

fn show_commands() {
    use crate::schema::commands::dsl::*;

    let mut connection = establish_connection();
    let results = commands
        .load::<Command>(&mut connection)
        .expect("Error loading commands");

    println!("Displaying {} commands", results.len());
    for command in results {
        println!("{:?}", command);
    }
}
