// commands and callbacks

use log::{info, debug};
use rand::{thread_rng, Rng, prelude::StdRng, SeedableRng};
use serenity::{framework::standard::{macros::{group, check, command, hook}, Args, CommandOptions, Reason, CommandResult}, client::Context, model::channel::Message};

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
        Err(why) => debug!("Command '{}' returned error {:?}", command_name, why),
    }
}

// commands 

// TODO: add help command

#[group]
#[commands(ping, roll, flip, git)]
pub struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}

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
#[command]
async fn git(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, GITHUB_URL).await?;
    Ok(())
}