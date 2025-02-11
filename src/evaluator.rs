use askama::Template;
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use schemars::schema_for;
use std::error::Error;

use crate::evaluation::Evaluation;

#[derive(Template)]
#[template(path = "system_prompt.md.jinja")]
struct SystemPromptTemplate {
    commit_message: String
}

#[derive(Template)]
#[template(path = "user_message.md.jinja")]
struct UserMessageTemplate {
    file_contents: String,
    code_diff: String,
    file_path: String,
}

/// # Errors
/// # Panics
pub async fn evaluate_file_change(
    commit_message: &str,
    file_path: &str,
    file_contents: &str,
    diff: &str,
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
            .content(construct_system_prompt(commit_message))
            .build()?
            .into()])
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(construct_user_message(file_path, file_contents, diff))
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

fn construct_system_prompt(commit_message: &str) -> String {
    let system_message = SystemPromptTemplate {
        commit_message: commit_message.into(),
    };
    system_message.render().unwrap()
}

fn construct_user_message(file_path: &str, file_contents: &str, diff: &str) -> String {
    let system_message = UserMessageTemplate {
        file_path: file_path.into(),
        file_contents: file_contents.into(),
        code_diff: diff.into(),
    };
    system_message.render().unwrap()
}

