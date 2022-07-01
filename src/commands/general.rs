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

// TODO: Clean this up and make the output look nicer, like with embeds or something
const GITHUB_URL: &'static str = "https://github.com/ChloeBangBang/chloebot";
/// Prints the github url this bot is hosted at!
#[command]
async fn git(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, GITHUB_URL).await?;
    Ok(())
}

// TODO: make a separate category for external api calls
/// Gets active Dead by Daylight shrine of secrets information
#[command]
async fn shrines(ctx: &Context, msg: &Message) -> CommandResult {
    let resp = reqwest::get("https://api.nightlight.gg/v1/shrine?pretty=true").await?.text().await?;
    msg.channel_id.say(ctx, resp).await?;

    Ok(())
}