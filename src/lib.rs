use std::{sync::{Arc, RwLock}, time::{Instant, Duration}};

use serenity::{
    prelude::*,
    async_trait, 
    model::gateway::Ready, client::bridge::gateway::ShardManager, 
};

use log::{info, error};

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

#[derive(Clone)]
pub struct Uptime {
    inst: Arc<RwLock<Instant>>,
}

impl Uptime {
    pub fn get_uptime(&self) -> Duration {
        match self.inst.read() {
            Ok(instant) => {
                return instant.elapsed();
            },
            Err(poison_lock) => {
                error!("Uptime lock poisoned!");
                return poison_lock.into_inner().elapsed();
            }
        }
    }
    pub fn new() -> Self {
        Self {
            inst: Arc::new(RwLock::new(Instant::now()))
        }
    }
}

impl TypeMapKey for Uptime {
    type Value = Uptime;
}