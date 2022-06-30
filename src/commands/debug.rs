use chloebot::ShardManagerContainer;
use serenity::{
    prelude::*, 
    framework::standard::{
        macros::command, CommandResult
    }, model::channel::Message, 
    client::bridge::gateway::ShardId
};

use crate::commands::OWNER_CHECK;

/// Responds with shard latency to discord servers
#[command]
#[checks(Owner)]
async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
    // from serenity example 5:
    // The shard manager is an interface for mutating, stopping, restarting, 
    // and retrieing information about shards
    let data = ctx.data.read().await;

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            msg.channel_id.say(ctx, "Failed to get the shard manager").await?;
            
            return Ok(());
        }
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            msg.channel_id.say(ctx, "No shard found").await?;
            
            return Ok(());
        }
    };

    match runner.latency {
        Some(dur) => {
            msg.channel_id.say(ctx, &format!("The shard latency is {:.3}ms", dur.as_secs_f64() * 1000.0)).await?;
        }
        None => {
            msg.channel_id.say(ctx, "Failed to get shard latency").await?;
        }
    }
    Ok(())
}