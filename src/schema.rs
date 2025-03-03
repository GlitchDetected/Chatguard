table! {
    afkusers (user_id) {
        user_id -> Varchar,
        message -> Varchar,
        duration -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    autoroles (guild_id) {
        guild_id -> Varchar,
        role_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    commands (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    prefixes (guild_id) {
        guild_id -> Varchar,
        guildprefix -> Varchar,
    }
}

table! {
    ranks (user_id, guild_id) {
        user_id -> Varchar,
        guild_id -> Varchar,
        xp -> Int4,
        level -> Int4,
        last_daily -> Nullable<Timestamp>,
        bg_color -> Varchar,
        bar_color -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tickets (ticket_id) {
        ticket_id -> Varchar,
        user_id -> Varchar,
        channel_id -> Varchar,
        guild_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    rankconfigs (guild_id) {
        guild_id -> Varchar,
        rankchannel -> Nullable<Varchar>,
        rankconfigure -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    ticketconfigs (guild_id) {
        guild_id -> Varchar,
        ticket_logs_channel_id -> Varchar,
        staff_news_channel_id -> Varchar,
        create_tickets_channel_id -> Varchar,
        user_tickets_category_id -> Varchar,
        ticketconfigure -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    uptimes (id) {
        id -> Int4,
        botuptime -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

// joinable!(views -> users (user_id));
// joinable!(views -> videos (video_id));
// The joinable! macro in Diesel defines foreign key relationships, 
// allowing seamless SQL joins between tables. In this case, 
// it enables joining views with users via user_id and views with videos via video_id for efficient querying.

allow_tables_to_appear_in_same_query!(
    afkusers,
    autoroles,
    commands,
    prefixes,
    ranks,
    tickets,
    rankconfigs,
    ticketconfigs,
    uptimes
);
