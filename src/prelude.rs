use std::io::Stdout;

use anyhow::Error;
use ratatui::{backend::CrosstermBackend, Terminal};

pub type AppError = Error;
pub type AppResult<T = ()> = Result<T, AppError>;
pub type AppBackEndTerminal = Terminal<CrosstermBackend<Stdout>>;
pub type Selection = Option<u16>;
pub type TermNum = u16;

pub use crate::constants;
pub use std::iter;
