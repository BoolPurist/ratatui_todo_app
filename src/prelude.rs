use std::io::Stdout;

use anyhow::Error;
use ratatui::{backend::CrosstermBackend, Terminal};

pub type AppError = Error;
pub type AppResult<T = ()> = Result<T, AppError>;
pub type AppBackEndTerminal = Terminal<CrosstermBackend<Stdout>>;
