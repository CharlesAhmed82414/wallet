use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum BroadcastError {
    ConflictingTransaction { message: String },
}

impl fmt::Display for BroadcastError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConflictingTransaction { message } => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for BroadcastError {}
