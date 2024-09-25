use askama::Template;
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs,
        CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MessageScore {
    Complete,
    Adequate,
    Incomplete,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone)]
pub struct Evaluation {
    pub message: MessageScore,
    pub reason: String,
    pub suggestion: String,
}

#[derive(Template)]
#[template(path = "prompt.txt")]
struct PromptTemplate {
    commit_message: String,
    code_diff: String
}


/// Evaluate a commit message and diff to determine if it is complete, adequate, or incomplete.
/// # Errors
/// # Panics
pub async fn evaluate_commit(message: &str, diff: &str) -> Result<Evaluation, Box<dyn Error>> {
    let mut json_schema = serde_json::to_value(schema_for!(Evaluation))?;
    json_schema["additionalProperties"] = serde_json::Value::Bool(false);

    let response_format = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            description: Some("Description".to_string()),
            name: "SchemaName".to_string(),
            strict: Some(true),
            schema: Some(json_schema),
        },
    };

    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o-2024-08-06")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(construct_system_message(message, diff))
                .build()?
                .into(),
        ])
        .response_format(response_format)
        .build()?;

    let response = client.chat().create(request).await?;
    let choice = response.choices.first().unwrap();

    Ok(serde_json::from_str(
        choice.message.content.as_ref().unwrap().as_str(),
    )?)
}

fn construct_system_message(message: &str, diff: &str) -> String {
    let system_message = PromptTemplate {
        commit_message: message.to_string(),
        code_diff: diff.to_string(),
    };
    system_message.render().unwrap()
}
