use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurrentView {
    TodoList,
    TodoCreation,
}

impl TryFrom<String> for CurrentView {
    type Error = AppError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "TodoList" => Ok(Self::TodoList),
            "TodoAdd" => Ok(Self::TodoCreation),
            _ => Err(anyhow!(
                "Word \"{}\" is not a valid keyword for a start view",
                value
            )),
        }
    }
}

impl Default for CurrentView {
    fn default() -> Self {
        Self::TodoList
    }
}
