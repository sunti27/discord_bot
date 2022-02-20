use std::collections::HashSet;
use serenity::client::Context;
use serenity::framework::standard::{Args, CommandGroup, CommandResult, help_commands, HelpOptions};
use serenity::framework::standard::macros::help;
use serenity::model::channel::Message;
use serenity::model::id::UserId;


#[help]
async fn help_command(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;

    Ok(())
}