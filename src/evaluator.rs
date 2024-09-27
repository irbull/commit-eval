use askama::Template;
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use conventional_commits::ConventionalCommit;
use schemars::schema_for;
use std::error::Error;

use crate::evaluation::Evaluation;

#[derive(Template)]
#[template(path = "system_prompt.md.jinja")]
struct SystemPromptTemplate {
    conventional_commit: Option<ConventionalCommit>,
}

#[derive(Template)]
#[template(path = "user_message.md.jinja")]
struct UserMessageTemplate {
    commit_message: String,
    code_diff: String,
}

/// Evaluate a commit message and diff to determine if it is complete, adequate, or incomplete.
/// # Errors
/// # Panics
pub async fn evaluate_commit(
    message: &str,
    diff: &str,
    conventional_commit: Option<ConventionalCommit>,
) -> Result<Evaluation, Box<dyn Error>> {
    let mut json_schema = serde_json::to_value(schema_for!(Evaluation))?;
    json_schema["additionalProperties"] = serde_json::Value::Bool(false);
    let response_format = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            description: Some("Evaluation Response".to_string()),
            name: "Evaluation".to_string(),
            strict: Some(true),
            schema: Some(json_schema),
        },
    };

    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o-2024-08-06")
        .messages([ChatCompletionRequestSystemMessageArgs::default()
            .content(construct_system_prompt(conventional_commit))
            .build()?
            .into()])
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(construct_user_message(message, diff))
            .build()?
            .into()])
        .response_format(response_format)
        .build()?;

    let response = client.chat().create(request).await?;
    let choice = response.choices.first().unwrap();

    Ok(serde_json::from_str(
        choice.message.content.as_ref().unwrap().as_str(),
    )?)
}

fn construct_system_prompt(conventional_commit: Option<ConventionalCommit>) -> String {
    let system_message = SystemPromptTemplate {
        conventional_commit,
    };
    system_message.render().unwrap()
}

fn construct_user_message(message: &str, diff: &str) -> String {
    let system_message = UserMessageTemplate {
        commit_message: message.into(),
        code_diff: diff.into(),
    };
    system_message.render().unwrap()
}

#[allow(clippy::unnecessary_wraps)]
mod filters {
    pub fn display_some<T>(value: &Option<T>) -> askama::Result<String>
    where
        T: std::fmt::Display,
    {
        Ok(match value {
            Some(value) => value.to_string(),
            None => String::new(),
        })
    }
}
