use rand::{thread_rng, Rng, prelude::StdRng, SeedableRng};
use serenity::{
    prelude::*, 
    framework::standard::{
        macros::command, CommandResult, Args
    }, model::channel::Message
};

// TODO: dnd dice syntax
// regex here for safe keeping :)
const REGEX: &str = r#"( ?\+? ?(\d+)?d(\d+))+ ?([\+\-] ?\d+)?"#;

/// Flips a coin
#[command]
pub async fn flip(ctx: &Context, msg: &Message) -> CommandResult {
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
pub async fn roll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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