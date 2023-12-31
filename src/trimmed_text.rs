use crate::prelude::*;
use derive_more::{AsRef, Deref, Display, Into};
use serde::{Deserialize, Serialize};

#[derive(Debug, AsRef, Deref, Into, Display, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct TrimmedText(String);

impl TrimmedText {
    pub fn new(content: String) -> AppResult<Self> {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            Err(anyhow!(
                "Text content must not be empty or only whitespaces"
            ))
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }
}

impl TryFrom<String> for TrimmedText {
    type Error = AppError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
