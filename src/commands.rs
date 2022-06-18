// commands and callbacks

use std::collections::HashSet;

use chloebot::ShardManagerContainer;
use log::{info, warn};
use rand::{thread_rng, Rng, prelude::*, SeedableRng};
use serenity::{
    framework::standard::{
        macros::{group, check, command, hook, help}, 
        Args, CommandOptions, Reason, CommandResult, HelpOptions, help_commands, CommandGroup
    }, client::{Context, bridge::gateway::ShardId}, 
    model::{channel::Message, id::UserId}
};

// hooks

// checks to make sure the owner of the bot is the one using the command
// the example doc for this uses a hardcoded ID, which is bad, 
// but I can't figure out how to pass arguments to this right now
// so I'm just going to hardcode my userid and call it a day
#[check]
#[name = "Owner"]
pub async fn owner_check(_: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> Result<(), Reason> {
    if msg.author.id != 159819497216278529 {
        return Err(Reason::User("Lacked owner permission".to_string()));
    }
    Ok(())
}

#[hook]
pub async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!("Got command '{}' by user '{}'", command_name, msg.author.name);

    true
}

#[hook]
pub async fn after(_: &Context, _: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => info!("Processed command '{}'", command_name),
        Err(why) => warn!("Command '{}' returned error {:?}", command_name, why),
    }
}

// commands 

#[help]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
async fn help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[group]
#[commands(ping, git)]
pub struct General;

#[group]
#[commands(roll, flip)]
pub struct Random;

#[group]
#[commands(latency)]
pub struct Debug;

/// Pong!
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}

/// Flips a coin
#[command]
async fn flip(ctx: &Context, msg: &Message) -> CommandResult {
    let flip: &str = match thread_rng().gen_bool(0.5) {
        true => {
            "Heads!"
        },
        false => {
            "Tails!"
        }
    };
    msg.channel_id.say(&ctx.http, flip).await?;
    Ok(())
}

// TODO: add case for DnD dice syntax (i.e. 4d6 + 5)
/// Rolls a random number from 1 to an optional upper bound, defaulting to 100
#[command]
async fn roll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // using thread_rng here causes the command proc macro to break for reasons I do not understand
    // StdRng::from_entropy() bypasses this :)
    let mut rng = StdRng::from_entropy();
    // upper bounds on the roll
    let upper: usize;

    // take an upper bounds from the command argument
    if let Ok(x) = args.trimmed().parse::<usize>() {
        upper = x;
    } 
    // otherwise, default to 100
    else {
        upper = 100;
    }

    let roll = rng.gen_range(1..=upper);

    msg.channel_id.say(&ctx.http, roll).await?;

    Ok(())
}

// TODO: Clean this up and make the output look nicer
const GITHUB_URL: &'static str = "https://github.com/ChloeBangBang/chloebot";
/// Prints the github url this bot is hosted at!
#[command]
async fn git(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, GITHUB_URL).await?;
    Ok(())
}

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