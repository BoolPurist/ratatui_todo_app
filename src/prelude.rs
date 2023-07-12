use std::io::Stdout;

use ratatui::{backend::CrosstermBackend, Terminal};

pub type AppError = anyhow::Error;
pub type AppResult<T = ()> = Result<T, AppError>;
pub type AppBackEndTerminal = Terminal<CrosstermBackend<Stdout>>;
pub type Selection = Option<u16>;
pub type TermNum = u16;
pub type Todos = Vec<Todo>;

pub use crate::constants;
use crate::Todo;
pub use std::iter;
