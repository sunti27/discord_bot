use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandError, CommandResult, macros::{command, group}};
use pyo3::prelude::*;
use regex::Regex;
use lazy_static::lazy_static;
use pyo3::types::PyDict;
use crate::utils::validate;

#[group]
#[owners_only]
#[commands(exec)]
struct Execution;

#[command]
#[description = "Execute python code"]
async fn exec(ctx: &Context, msg: &Message) -> CommandResult {
    let content = cleanup_codeblock(&msg.content.trim_start_matches("!exec").trim());

    if content.trim().is_empty() {
        return Err(CommandError::from("Error: Missing arguments"));
    }

    let code = format_code(content);

    let output = Python::with_gil(|py| -> Result<Option<String>, CommandError> {
        let locals = PyDict::new(py);

        if let Err(why) = py.run(code.as_str(),None,Some(locals)) {
            return Err(CommandError::from(why.to_string()));
        }

        if let Some(output) = locals.get_item("output") {
            return Ok(output.to_string().into());
        }

        Ok(None)
    })?;

    validate(&ctx, &msg).await?;

    if let Some(output) = output {
        if !output.trim().is_empty() {
            msg.reply(&ctx, format_outut(output)).await?;
        }
    }

    Ok(())
}

fn cleanup_codeblock(content: &str) -> &str {
    // thank you https://regex101.com
    lazy_static! {
        static ref REGEX_MULTILINE: Regex = Regex::new(r"^```(?:py(?:thon)?)?((?:\n.*)+)\n```$").unwrap();
        static ref REGEX_ONELINE: Regex = Regex::new(r"^`(.*)`$").unwrap();
    }

    if let Some(captures) = REGEX_MULTILINE.captures(content) {
        captures.get(1).map(|v| v.as_str()).unwrap_or(content)
    } else if let Some(captures) = REGEX_ONELINE.captures(content) {
        captures.get(1).map(|v| v.as_str()).unwrap_or(content)
    } else {
        content
    }
}

fn format_code(content: &str) -> String {
    let mut code = String::new();

    code.push_str("from io import StringIO\n");
    code.push_str("from contextlib import redirect_stdout\n");

    code.push_str("_print = print\n");

    code.push_str("def print(*args, **kwargs):\n");
    code.push_str("    _print(*args, **kwargs, flush=True)\n");

    code.push_str("def func():\n");

    for line in content.split("\n") {
        code.push_str("    ");
        code.push_str(line);
        code.push_str("\n")
    }

    code.push_str("stdout = StringIO()\n");
    code.push_str("with redirect_stdout(stdout):\n");
    code.push_str("    func()\n");

    code.push_str("output = stdout.getvalue()");

    code
}

fn format_outut(output: String) -> String {
    format!("Output:\n```py\n> {}\n```", output)
}