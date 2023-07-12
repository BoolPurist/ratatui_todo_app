#[allow(unused_imports)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

use std::env::VarError;

pub use app_context::AppContext;
use app_context::CurrentView;
pub use todo::Todo;
pub use trimmed_text::TrimmedText;

pub mod constants;

mod app_context;
mod data_file_source;
mod drawing;
mod input;
mod prelude;
mod run;
mod setup;
mod todo;
mod trimmed_text;

use prelude::*;

fn main() -> AppResult<()> {
    setup::load_env_file();
    setup::set_up_logger();

    let mut terminal = setup::setup_terminal()?;
    let mut app = set_up_app()?;
    run::run(&mut app, &mut terminal)?;
    setup::restore_terminal(&mut terminal)?;
    Ok(())
}

fn set_up_app() -> AppResult<AppContext> {
    let data = data_file_source::provide_data()?;
    let mut app = AppContext::new(data.into_iter());

    let view: CurrentView = match std::env::var(constants::START_SCREEN) {
        Ok(to_resolve) => to_resolve.try_into(),
        Err(VarError::NotPresent) => Ok(CurrentView::default()),
        _ => Err(anyhow!("Invalid")),
    }
    .unwrap();

    app.current_view = view;
    Ok(app)
}
