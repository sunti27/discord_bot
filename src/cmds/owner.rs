use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{command, group}};

use lazy_static::lazy_static;
use regex::Regex;
use crate::utils::invalidate;

#[group]
#[commands(exec)]
struct Owner;


#[command]
#[description = "Execute python code"]
async fn exec(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(content) = cleanup_code(&msg.content) {
        // TODO: pyo3 black magic
    } else {
        invalidate(ctx, msg).await?;
    }

    Ok(())
}

fn cleanup_code(content: &str) -> Option<&str> {
    lazy_static! {
        static ref REGEX_MULTILINE: Regex = Regex::new(r"^!exec ```(?:py(?:thon)?)?((?:\n.*)+)\n```$").unwrap();
        static ref REGEX_ONELINE: Regex = Regex::new(r"^!exec `(.*)`$").unwrap();
    }

    if let Some(captures) = REGEX_MULTILINE.captures(content) {
        captures.get(1).map(|v| v.as_str())
    } else if let Some(captures) = REGEX_ONELINE.captures(content) {
        captures.get(1).map(|v| v.as_str())
    } else {
        None
    }
}