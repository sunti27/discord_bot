use serenity::client::Context;
use serenity::model::channel::{Message, Reaction, ReactionType};

const INVALID_REACTION: &str = "\u{274C}";
const DEBUG_REACTION: &str = "\u{1F50D}";
const VALID_REACTION: &str = "\u{2705}";

pub(crate) async fn invalidate(
    ctx: &Context,
    msg: &Message,
) -> serenity::Result<Reaction> {
    msg.react(ctx, ReactionType::Unicode(String::from(INVALID_REACTION))).await
}

pub(crate) async fn validate(
    ctx: &Context,
    msg: &Message,
) -> serenity::Result<Reaction> {
    msg.react(ctx, ReactionType::Unicode(String::from(VALID_REACTION))).await
}

pub(crate) async fn debug_info(
    ctx: &Context,
    msg: &Message,
    info: &str
) -> serenity::Result<()> {
    msg.react(ctx, ReactionType::Unicode(String::from(DEBUG_REACTION))).await?;

    if let Some(_reaction) = msg
        .await_reaction(&ctx)
        .filter(|r| r.emoji == ReactionType::Unicode(String::from(DEBUG_REACTION)))
        .author_id(msg.author.id.0)
        .timeout(tokio::time::Duration::from_secs(3 * 60))
        .await
    {

        let _result = &msg
            .channel_id
            .say(&ctx.http, format!("```\n{info}\n```"))
            .await?;
    }

    Ok(())
}