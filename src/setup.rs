use crate::prelude::*;

use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use dotenv;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::path::Path;
use std::{env, io};

const ENV_DATA: &str = ".env";
const PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub fn set_up_logger() {
    env_logger::init();
}

pub fn load_env_file() {
    if cfg!(debug_assertions) {
        let path: &Path = &Path::new(PROJECT_ROOT.into());
        let path = path.join(ENV_DATA);

        dotenv::from_path(path).expect("Could load env file for development.");
    }
}

pub fn setup_terminal() -> AppResult<AppBackEndTerminal> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

pub fn restore_terminal(terminal: &mut AppBackEndTerminal) -> AppResult<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}
