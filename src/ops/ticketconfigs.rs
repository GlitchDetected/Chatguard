use crate::args::{
    TicketConfigsCommand, 
    TicketConfigsSubcommand, 
    ConfigureTicket
};

use crate::db::establish_connection;
use crate::models::{NewTicketConfig, TicketConfig};
use diesel::prelude::*;

pub fn handle_ticketconfigs_command(ticketconfigs: TicketConfigsCommand) {
    let command = ticketconfigs.command;
    match command {
        TicketConfigsSubcommand::Configure(ticketconfig) => {
            configure_ticket(ticketconfig);
        }
        TicketConfigsSubcommand::Show => {
            show_ticketconfigs();
        }
    }
}

fn configure_ticket(ticketconfig: ConfigureTicket) {
    println!("Configuring ticket settings for guild: {}", ticketconfig.guild_id);
    use crate::schema::ticketconfigs::dsl::*;

    let mut connection = establish_connection();
    let new_ticketconfig = NewTicketConfig {
        guild_id: &ticketconfig.guild_id,
        ticket_logs_channel_id: &ticketconfig.ticket_logs_channel_id,
        staff_news_channel_id: &ticketconfig.staff_news_channel_id,
        create_tickets_channel_id: &ticketconfig.create_tickets_channel_id,
        user_tickets_category_id: &ticketconfig.user_tickets_category_id,
        ticketconfigure: ticketconfig.ticketconfigure,
    };

    diesel::insert_into(ticketconfigs)
        .values(&new_ticketconfig)
        .on_conflict(guild_id)
        .do_update()
        .set((
            ticket_logs_channel_id.eq(&ticketconfig.ticket_logs_channel_id),
            staff_news_channel_id.eq(&ticketconfig.staff_news_channel_id),
            create_tickets_channel_id.eq(&ticketconfig.create_tickets_channel_id),
            user_tickets_category_id.eq(&ticketconfig.user_tickets_category_id),
            ticketconfigure.eq(ticketconfig.ticketconfigure),
        ))
        .execute(&mut connection)
        .expect("Error configuring ticket settings");
}

fn show_ticketconfigs() {
    use crate::schema::ticketconfigs::dsl::*;

    let mut connection = establish_connection();
    let results = ticketconfigs
        .load::<TicketConfig>(&mut connection)
        .expect("Error loading ticket configurations");

    println!("Displaying {} ticket configurations", results.len());
    for config in results {
        println!(
            "Guild ID: {}, Ticket Logs: {}, Staff News: {}, Create Tickets: {}, User Tickets Category: {}, Configured: {}",
            config.guild_id,
            config.ticket_logs_channel_id,
            config.staff_news_channel_id,
            config.create_tickets_channel_id,
            config.user_tickets_category_id,
            config.ticketconfigure
        );
    }
}
