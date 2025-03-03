use clap::{
    Args, 
    Parser, 
    Subcommand
};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct ChatguardArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, update, delete or show afk users
    Afk(AfkCommand),

    /// Create, update, delete or show autoroles
    Autorole(AutoroleCommand),

    /// Create, update, delete or show commands
    Commands(CommandsCommand),

    /// Create, update, delete or show guild prefix
    Prefix(PrefixCommand),

    /// Create, update, delete or show user rank
    Rank(RankCommand),

    /// Create, update, delete or show guild rank configs
    RankConfigs(RankConfigsCommand),

    /// Create, update, delete or show guild ticket configs
    TicketConfigs(TicketConfigsCommand),

    /// Create, update, delete or show guild tickets
    Tickets(TicketsCommand),

    /// Create, update, delete or show bot uptime
    Uptime(UptimeCommand),
}

#[derive(Debug, Args)]
pub struct AfkCommand {
    #[clap(subcommand)]
    pub command: AfkSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AfkSubcommand {
    Create(CreateAfk),
    Delete(DeleteEntity),
    Show,
}

#[derive(Debug, Args)]
pub struct CreateAfk {
    pub user_id: String,
    pub message: String,
    pub duration: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Args)]
pub struct AutoroleCommand {
    #[clap(subcommand)]
    pub command: AutoroleSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AutoroleSubcommand {
    Create(CreateAutorole),
    Update(UpdateAutorole),
    Delete(DeleteEntity),
    Show,
}

#[derive(Debug, Args)]
pub struct CreateAutorole {
    pub guild_id: String,
    pub role_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Args)]
pub struct UpdateAutorole {
    pub guild_id: String,
    pub role_id: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Args)]
pub struct CommandsCommand {
    #[clap(subcommand)]
    pub command: CommandsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CommandsSubcommand {
    Create(CreateCommand),
    Update(UpdateCommand),
    Delete(DeleteEntity),
    Show,
}

#[derive(Debug, Args)]
pub struct CreateCommand {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Args)]
pub struct UpdateCommand {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Args)]
pub struct PrefixCommand {
    #[clap(subcommand)]
    pub command: PrefixSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum PrefixSubcommand {
    Create(CreatePrefix),
    Update(UpdatePrefix),
    Delete(DeleteEntity),
    Show,
}

#[derive(Debug, Args)]
pub struct CreatePrefix {
    pub guild_id: String,
    pub guildprefix: String,
}

#[derive(Debug, Args)]
pub struct UpdatePrefix {
    pub guild_id: String,
    pub guildprefix: String,
}

#[derive(Debug, Args)]
pub struct RankCommand {
    #[clap(subcommand)]
    pub command: RankSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum RankSubcommand {
    Create(CreateRank),
    Update(UpdateRank),
    Delete(DeleteEntity),
    Show,
}

#[derive(Debug, Args)]
pub struct CreateRank {
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

#[derive(Debug, Args)]
pub struct UpdateRank {
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

#[derive(Debug, Args)]
pub struct RankConfigsCommand {
    #[clap(subcommand)]
    pub command: RankConfigsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum RankConfigsSubcommand {
    Configure(ConfigureRank),
    Show,
}

#[derive(Debug, Args)]
pub struct ConfigureRank {
    pub guild_id: String,
    pub rankchannel: Option<String>,
    pub rankconfigure: bool,
}

#[derive(Debug, Args)]
pub struct TicketConfigsCommand {
    #[clap(subcommand)]
    pub command: TicketConfigsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum TicketConfigsSubcommand {
    Configure(ConfigureTicket),
    Show,
}

#[derive(Debug, Args)]
pub struct ConfigureTicket {
    pub guild_id: String,
    pub ticket_logs_channel_id: String,
    pub staff_news_channel_id: String,
    pub create_tickets_channel_id: String,
    pub user_tickets_category_id: String,
    pub ticketconfigure: bool,
}

#[derive(Debug, Args)]
pub struct TicketsCommand {
    #[clap(subcommand)]
    pub command: TicketsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum TicketsSubcommand {
    Create(CreateTicket),
    Show,
}

#[derive(Debug, Args)]
pub struct CreateTicket {
    pub ticket_id: String,
    pub user_id: String,
    pub channel_id: String,
    pub guild_id: String,
}

#[derive(Debug, Args)]
pub struct UptimeCommand {
    #[clap(subcommand)]
    pub command: UptimeSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UptimeSubcommand {
    Update(UpdateUptime),
    Create(CreateUptime),
    Delete(DeleteEntity),
    Show,
}

#[derive(Debug, Args)]
pub struct UpdateUptime {
    pub id: i32,
    pub botuptime: i64,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Args)]
pub struct CreateUptime {
    pub id: i32,
    pub botuptime: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    pub id: i32,
}