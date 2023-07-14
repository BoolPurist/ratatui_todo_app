use std::io::Stdout;

use ratatui::{backend::CrosstermBackend, Terminal};

pub type AppError = anyhow::Error;
pub type AppResult<T = ()> = Result<T, AppError>;
pub type AppBackEnd = CrosstermBackend<Stdout>;
pub type AppBackEndTerminal = Terminal<AppBackEnd>;
pub type Selection = u16;
pub type Todos = Vec<Todo>;

pub use crate::constants;
use crate::Todo;
pub use std::iter;
