use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use std::env;
mod calculations;
mod commands;
const BOT_PREFIX: &str = "*";
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message: Message) {
        if message.content.starts_with(BOT_PREFIX) {
            match message.content.as_str() {
                "*primos" => {
                    let builder = commands::primos();
                    let _ = message
                        .channel_id
                        .send_message(context.http(), builder)
                        .await;
                }
                &_ => (),
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.id);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::all();
    let mut bot_client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    println!("Hello, world!");
    if let Err(error) = bot_client.start().await {
        println!("Error starting due to: {error}");
    }

    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(error) = bot_client.start().await {
        println!("Client connection error due to: {error:?}");
    }
}
