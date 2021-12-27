use dotenv::dotenv;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{macros::group, StandardFramework};
use serenity::model::gateway::{Activity, Ready};
use std::env;
mod commands;
use commands::{ping::*,sus::*};
#[group]
#[commands(ping,sus)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Logged in as {}", ready.user.name);
        ctx.set_activity(Activity::playing("Rust")).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("?"))
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("token").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
