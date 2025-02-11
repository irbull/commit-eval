use core::fmt;
use std::fmt::{Display, Formatter};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CodeScore {
    Excellent,
    Good,
    Acceptable,
    NeedsImprovement,
    Poor,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone)]
pub struct Evaluation {
    pub score: CodeScore,
    pub reason: String,
    //pub suggestions: String,
}

impl Display for CodeScore {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Excellent => write!(f, "Excellent"),
            Self::Good => write!(f, "Good"),
            Self::Acceptable => write!(f, "Acceptable"),
            Self::NeedsImprovement => write!(f, "Needs Improvement"),
            Self::Poor => write!(f, "Poor"),
        }
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Score: {}\nReason: {}\n",
            self.score, self.reason
        )
    }
}
