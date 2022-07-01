use std::time::Duration;

use chloebot::{ShardManagerContainer, Uptime};
use serenity::{
    prelude::*, 
    framework::standard::{
        macros::command, CommandResult
    }, model::channel::Message, 
    client::bridge::gateway::ShardId
};

use log::error;

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
            error!("Failed to get the shard manager");
            msg.channel_id.say(ctx, "Failed to get the shard manager").await?;
            
            return Ok(());
        }
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            error!("No shard found in Shard Manager");
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

const DAYS_TO_SECS: u64 = 86400;
const HOURS_TO_SECS: u64 = 3600;
/// Convert a [Duration] to a days, hours, minutes, seconds tuple
fn duration_to_dhms(dur: Duration) -> (u64, u64, u64, u64) {
    let (d, h, m, s);
    let total_seconds = dur.as_secs();
    let mut rem: u64 = total_seconds;

    d = rem / DAYS_TO_SECS;
    rem %= DAYS_TO_SECS;

    h = rem / HOURS_TO_SECS;
    rem %= HOURS_TO_SECS;

    m = rem / 60;
    rem %= 60;

    s = rem;
    (d, h, m, s)
}

/// Returns the time the bot's been running
#[command]
async fn uptime(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    match data.get::<Uptime>() {
        Some(up) => {
            let (d, h, m, s) = duration_to_dhms(up.get_uptime());
            msg.channel_id.say(ctx, format!("{}:{}:{}:{}", d,h,m,s)).await?;
        },
        None => {
            error!("Failed to get uptime!");
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duration_to_dhms_test() {
        assert_eq!(duration_to_dhms(Duration::from_secs(783399)), (9, 1, 36, 39));
        assert_eq!(duration_to_dhms(Duration::from_secs(936123)), (10, 20, 2, 3));
        assert_eq!(duration_to_dhms(Duration::from_secs(379466)), (4, 9, 24, 26));
        assert_eq!(duration_to_dhms(Duration::from_secs(79605)), (0, 22, 6, 45));
        assert_eq!(duration_to_dhms(Duration::from_secs(2994)), (0, 0, 49, 54));
        assert_eq!(duration_to_dhms(Duration::from_secs(32)), (0, 0, 0, 32));
    }
}