mod cmds;
mod event;
mod help;
mod utils;

use serenity::client::{Client, validate_token};
use serenity::framework::standard::StandardFramework;

use std::env;

use cmds::{
    moderation, owner
};
use event::Handler;

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .help(&help::HELP_COMMAND)
        .group(&moderation::MODERATION_GROUP)
        .group(&owner::OWNER_GROUP);

    let token = env::var("DISCORD_TOKEN")
        .expect("Token not found");

    validate_token(&token)
        .expect("Token invalid");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}

