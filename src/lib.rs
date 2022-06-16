use std::sync::Arc;

use serenity::{
    prelude::*,
    async_trait, 
    model::gateway::Ready, client::bridge::gateway::ShardManager, 
};

use log::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}