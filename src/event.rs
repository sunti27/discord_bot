use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::CommandError;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use crate::utils::{invalidate, debug_info};
use serenity::framework::standard::macros::hook;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, bot_data: Ready) {
        println!("Connected as {}", bot_data.user.name);
    }
}

#[hook]
pub async fn after_hook(ctx: &Context, msg: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    if let Err(why) = error {
        let _result = invalidate(ctx, msg).await;

        if cmd_name == "exec" {
            let info = why.to_string();

            let _result = debug_info(ctx, msg, &info).await;
        }
    }
}