#[macro_use]
extern crate log;

use std::collections::HashSet;
use std::sync::Arc;

use chloebot::{Handler, ShardManagerContainer};
use commands::{GENERAL_GROUP, DEBUG_GROUP, HELP, RANDOM_GROUP};
use config::Config;
use serenity::{http::Http, framework::StandardFramework};
use serenity::prelude::*;

mod config;
mod commands;
mod lib;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = Config::get().expect("failed to fetch config");

    let http = Http::new(&config.get_token());

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            // I suspect this formatting is unnecessary given the bot only has one owner (me)
            // but I'm too tired to mess with it right now
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }

            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix(config.get_prefix())
            .owners(owners)
            .case_insensitivity(config.is_case_insensitive())
        ).help(&HELP)
        .group(&GENERAL_GROUP)
        .group(&RANDOM_GROUP)
        .group(&DEBUG_GROUP)
        // hook to execute before a command gets run
        .before(commands::before)
        // hook to execute after a command gets run
        .after(commands::after);

        let intents = GatewayIntents::all();
        let mut client = Client::builder(&config.get_token(), intents)
            .event_handler(Handler)
            .framework(framework)
            .await
            .expect("Error creating client");

        {
            let mut data = client.data.write().await;
            data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        }
        
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }        
}
