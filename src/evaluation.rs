use core::fmt;
use std::fmt::{Display, Formatter};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MessageScore {
    Complete,
    Adequate,
    Incomplete,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone)]
pub struct Evaluation {
    pub score: MessageScore,
    pub reason: String,
    pub suggested_title: String,
    pub suggested_body: String,
}

impl Display for MessageScore {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            MessageScore::Complete => write!(f, "✅ Complete"),
            MessageScore::Adequate => write!(f, "🟨 Adequate"),
            MessageScore::Incomplete => write!(f, "❌ Incomplete"),
        }
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}\n", self.score)?;
        match self.score {
            MessageScore::Complete => write!(f, "\nReason: {}", self.reason),
            _ => write!(
                f,
                "\nReason: {}\n---\n{}\n\n{}",
                self.reason, self.suggested_title, &self.suggested_body
            ),
        }
    }
}


