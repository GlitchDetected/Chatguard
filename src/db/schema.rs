// @generated automatically by Diesel CLI.

diesel::table! {
    afkusers (user_id) {
        user_id -> Varchar,
        message -> Varchar,
        duration -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    autoroles (guild_id) {
        guild_id -> Varchar,
        role_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    commands (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    prefixes (guild_id) {
        guild_id -> Varchar,
        guildprefix -> Varchar,
    }
}

diesel::table! {
    rankconfigs (guild_id) {
        guild_id -> Varchar,
        rankchannel -> Nullable<Varchar>,
        rankconfigure -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
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

diesel::table! {
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

diesel::table! {
    tickets (ticket_id) {
        ticket_id -> Varchar,
        user_id -> Varchar,
        channel_id -> Varchar,
        guild_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    uptimes (id) {
        id -> Int4,
        botuptime -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    blocked_guilds (guild) {
        guild -> BigInt,
    }
}

diesel::table! {
    blocked_users (user) {
        user -> BigInt,
    }
}

diesel::table! {
    settings (guild) {
        guild -> BigInt,
        game -> Nullable<Integer>,
        prefix -> Nullable<Text>,
    }
}

diesel::table! {
    subscriptions (game, channel, tags) {
        game -> Integer,
        channel -> BigInt,
        tags -> Text,
        guild -> Nullable<BigInt>,
        events -> Integer,
    }
}

diesel::table! {
    subscriptions_exclude_mods (game, channel, mod_id) {
        game -> Integer,
        channel -> BigInt,
        guild -> Nullable<BigInt>,
        mod_id -> Integer,
    }
}

diesel::table! {
    subscriptions_exclude_users (game, channel, user) {
        game -> Integer,
        channel -> BigInt,
        guild -> Nullable<BigInt>,
        user -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    afkusers,
    autoroles,
    commands,
    prefixes,
    rankconfigs,
    ranks,
    ticketconfigs,
    tickets,
    uptimes,
    blocked_guilds,
    blocked_users,
    settings,
    subscriptions,
    subscriptions_exclude_mods,
    subscriptions_exclude_users,
);
