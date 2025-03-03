use crate::schema::afkusers;
use crate::schema::autoroles;
use crate::schema::commands;
use crate::schema::prefixes;
use crate::schema::ranks;
use crate::schema::tickets;
use crate::schema::rankconfigs;
use crate::schema::ticketconfigs;
use crate::schema::uptimes;

#[derive(Insertable)]
#[table_name = "afkusers"]
pub struct NewAfk<'a> {
    pub user_id: &'a str,
    pub message: &'a str,
    pub duration: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Afk {
    pub user_id: String,
    pub message: String,
    pub duration: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "autoroles"]
pub struct NewAutoRole<'a> {
    pub guild_id: &'a str,
    pub role_id: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct AutoRole {
    pub guild_id: String,
    pub role_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "commands"]
pub struct NewCommand<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Queryable, Debug)]
pub struct Command {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "prefixes"]
pub struct NewPrefix<'a> {
    pub guild_id: &'a str,
    pub guildprefix: &'a str,
}

#[derive(Queryable, Debug)]
pub struct Prefix {
    pub guild_id: String,
    pub guildprefix: String,
}

#[derive(Insertable)]
#[table_name = "ranks"]
pub struct NewRank<'a> {
    pub user_id: &'a str,
    pub guild_id: &'a str,
    pub xp: i32,
    pub level: i32,
    pub last_daily: Option<chrono::NaiveDateTime>,
    pub bg_color: &'a str,
    pub bar_color: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Rank {
    pub user_id: String,
    pub guild_id: String,
    pub xp: i32,
    pub level: i32,
    pub last_daily: Option<chrono::NaiveDateTime>,
    pub bg_color: String,
    pub bar_color: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "tickets"]
pub struct NewTicket<'a> {
    pub ticket_id: &'a str,
    pub user_id: &'a str,
    pub channel_id: &'a str,
    pub guild_id: &'a str,
}

#[derive(Queryable, Debug)]
pub struct Ticket {
    pub ticket_id: String,
    pub user_id: String,
    pub channel_id: String,
    pub guild_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "uptimes"]
pub struct NewUptime {
    pub id: i32,
    pub botuptime: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Uptime {
    pub id: i32,
    pub botuptime: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "rankconfigs"]
pub struct NewRankConfig<'a> {
    pub guild_id: &'a str,
    pub rankchannel: Option<&'a str>,
    pub rankconfigure: bool,
}

#[derive(Queryable, Debug)]
pub struct RankConfig {
    pub guild_id: String,
    pub rankchannel: Option<String>,
    pub rankconfigure: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "ticketconfigs"]
pub struct NewTicketConfig<'a> {
    pub guild_id: &'a str,
    pub ticket_logs_channel_id: &'a str,
    pub staff_news_channel_id: &'a str,
    pub create_tickets_channel_id: &'a str,
    pub user_tickets_category_id: &'a str,
    pub ticketconfigure: bool,
}

#[derive(Queryable, Debug)]
pub struct TicketConfig {
    pub guild_id: String,
    pub ticket_logs_channel_id: String,
    pub staff_news_channel_id: String,
    pub create_tickets_channel_id: String,
    pub user_tickets_category_id: String,
    pub ticketconfigure: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
