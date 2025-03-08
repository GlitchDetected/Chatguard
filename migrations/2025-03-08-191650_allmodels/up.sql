CREATE TABLE afkusers (
    user_id VARCHAR PRIMARY KEY,
    message VARCHAR NOT NULL,
    duration INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE autoroles (
    guild_id VARCHAR PRIMARY KEY,
    role_id VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE commands (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE prefixes (
    guild_id VARCHAR PRIMARY KEY,
    guildprefix VARCHAR NOT NULL
);

CREATE TABLE ranks (
    user_id VARCHAR NOT NULL,
    guild_id VARCHAR NOT NULL,
    xp INT NOT NULL DEFAULT 0,
    level INT NOT NULL DEFAULT 1,
    last_daily TIMESTAMP NULL,
    bg_color VARCHAR NOT NULL,
    bar_color VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, guild_id)
);

CREATE TABLE tickets (
    ticket_id VARCHAR PRIMARY KEY,
    user_id VARCHAR NOT NULL,
    channel_id VARCHAR NOT NULL,
    guild_id VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE rankconfigs (
    guild_id VARCHAR PRIMARY KEY,
    rankchannel VARCHAR NULL,
    rankconfigure BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE ticketconfigs (
    guild_id VARCHAR PRIMARY KEY,
    ticket_logs_channel_id VARCHAR NOT NULL,
    staff_news_channel_id VARCHAR NOT NULL,
    create_tickets_channel_id VARCHAR NOT NULL,
    user_tickets_category_id VARCHAR NOT NULL,
    ticketconfigure BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE uptimes (
    id SERIAL PRIMARY KEY,
    botuptime BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
