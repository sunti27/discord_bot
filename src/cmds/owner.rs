#![allow(unused)]
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandError, CommandResult, macros::{command, group}};


#[group]
#[owners_only]
struct Owner;

