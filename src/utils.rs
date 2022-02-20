use serenity::client::Context;
use serenity::model::channel::{Message, Reaction, ReactionType};
use std::future::Future;

const INVALID_REACTION: &str = "\u{274C}";

pub(crate) fn invalidate<'a>(
    ctx: &'a Context,
    msg: &'a Message,
) -> impl Future<Output = serenity::Result<Reaction>> + 'a {
    msg.react(ctx, ReactionType::Unicode(String::from(INVALID_REACTION)))
}
