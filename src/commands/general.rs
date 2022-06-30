use serenity::{
    prelude::*, 
    framework::standard::{
        macros::command, CommandResult
    }, model::channel::Message
};

/// Pong!
#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
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
