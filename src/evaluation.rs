use core::fmt;
use std::fmt::{Display, Formatter};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MessageScore {
    Excellent,
    Revisable,
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
            MessageScore::Excellent => write!(f, "✅ Excellent"),
            MessageScore::Revisable => write!(f, "🟨 Revisable"),
        }
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{}", self.score)?;
        match self.score {
            MessageScore::Excellent => write!(f, "\nReason: {}", self.reason),
            MessageScore::Revisable => write!(
                f,
                "\nReason: {}\n---\n{}\n\n{}",
                self.reason, self.suggested_title, &self.suggested_body
            ),
        }
    }
}
