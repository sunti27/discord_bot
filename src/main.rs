mod cmds;
mod event;
mod help;
mod utils;

use std::collections::HashSet;
use serenity::client::{Client, validate_token};
use serenity::framework::standard::StandardFramework;
use serenity::http::Http;
use std::env;

use cmds::{
    moderation, owner, execution
};
use event::{
    Handler,
    after_hook
};

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Token not found");

    validate_token(&token)
        .expect("Token invalid");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .configure(|c| c.owners(owners))
        .help(&help::HELP_COMMAND)
        .after(after_hook)
        .group(&moderation::MODERATION_GROUP)
        .group(&owner::OWNER_GROUP)
        .group(&execution::EXECUTION_GROUP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}

