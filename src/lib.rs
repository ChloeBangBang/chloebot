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

/// A container type for inserting into the client's data field, allowing it to be accessed from across all events and commands
/// or anywhere else that has access to the Context
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}