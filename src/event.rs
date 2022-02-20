use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, bot_data: Ready) {
        println!("Connected as {}", bot_data.user.name);
    }
}