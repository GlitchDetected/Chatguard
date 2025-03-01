mod ping;
mod google;
mod getavatar;
mod help;
mod issinfo;
mod serverinfo;
mod userinfo;
mod vote;
mod uptime;

use poise::Command;

pub fn getallcmd() -> Vec<Command<crate::state::State, Box<dyn std::error::Error + Send + Sync>>> {
    vec![
        ping::ping(),
        google::google(),
        getavatar::getavatar(),
        help::help(),
        issinfo::issinfo(),
        serverinfo::serverinfo(),
        userinfo::userinfo(),
        vote::vote(),
        uptime::uptime(),
    ]
}