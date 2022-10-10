use std::env;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.name == ctx.cache.current_user().name {
            return;
        }
        let mentions = msg.mentions.clone();
        for mention in mentions {
            let new_message = match mention.name.as_str() {
                "TheGodfatherCC" => "What a boss!",
                "cody19" => "💩💩🌮💩",
                &_ => "",
            };
            if !new_message.is_empty() {
                if let Err(why) = msg.reply(ctx.clone(), new_message).await {
                    println!("{}", why);
                }
            }
        }
        return;
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    println!("{}", msg.author.name);
    Ok(())
}
