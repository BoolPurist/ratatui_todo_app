use crate::prelude::*;
use derive_more::{AsRef, Deref, Display, Into};

#[derive(Debug, AsRef, Deref, Into, Display, PartialEq, Eq, Clone)]
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
