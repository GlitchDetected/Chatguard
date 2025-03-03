use crate::args::{
    TicketsCommand, 
    TicketsSubcommand, 
    CreateTicket
};

use crate::db::establish_connection;
use crate::models::{NewTicket, Ticket};
use diesel::prelude::*;

pub fn handle_tickets_command(tickets: TicketsCommand) {
    let command = tickets.command;
    match command {
        TicketsSubcommand::Create(ticket) => {
            create_ticket(ticket);
        }
        TicketsSubcommand::Show => {
            show_tickets();
        }
    }
}

fn create_ticket(ticket: CreateTicket) {
    println!("Creating ticket with ID: {}", ticket.ticket_id);
    use crate::schema::tickets::dsl::*;

    let mut connection = establish_connection();
    let new_ticket = NewTicket {
        ticket_id: &ticket.ticket_id,
        user_id: &ticket.user_id,
        channel_id: &ticket.channel_id,
        guild_id: &ticket.guild_id,
    };

    diesel::insert_into(tickets)
        .values(&new_ticket)
        .execute(&mut connection)
        .expect("Error creating ticket");
}

fn show_tickets() {
    use crate::schema::tickets::dsl::*;

    let mut connection = establish_connection();
    let results = tickets
        .load::<Ticket>(&mut connection)
        .expect("Error loading tickets");

    println!("Displaying {} tickets", results.len());
    for ticket in results {
        println!(
            "Ticket ID: {}, User ID: {}, Channel ID: {}, Guild ID: {}",
            ticket.ticket_id, ticket.user_id, ticket.channel_id, ticket.guild_id
        );
    }
}
