use std::collections::HashSet;

use serenity::{prelude::*, framework::standard::{macros::{group, check, hook, help}, CommandResult, Args, CommandOptions, Reason, HelpOptions, CommandGroup, help_commands}, model::{channel::Message, id::UserId}};

mod general;
mod random;
mod debug;

use general::*;
use random::*;
use debug::*;

// command groups

#[group]
#[commands(roll, flip)]
pub struct Random;

#[group]
#[commands(ping, git, shrines)]
pub struct General;

#[group]
#[commands(latency, uptime)]
pub struct Debug;

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

