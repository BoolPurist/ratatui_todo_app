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

mod app_context;
pub mod constants;
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

    let mut app = AppContext::new(
        [
            Todo::dev_new("Hello xxxxxxxxxxxx xxxx xxxxxxxx xxxxxxxx xxxxx xxxxxxxxx xxxxxx xxxxxx xxxxxx xxxxxxx xxxxxxx xxx222222xx"),
            Todo::dev_new("Hello"),
            Todo::dev_new("Hello xxxxxxxxxxxx xxxx xxxxxxxx xxxxxxxx xxxxx xxxxxxxxx xxxxxx xxxxxx xxxxxx xxxxxxx xxxxxxx xxx222222xx"),
        ]
        .into_iter(),
    );

    let view: CurrentView = match std::env::var(constants::START_SCREEN) {
        Ok(to_resolve) => to_resolve.try_into(),
        Err(VarError::NotPresent) => Ok(CurrentView::default()),
        _ => Err(anyhow!("Invalid")),
    }
    .unwrap();

    app.current_view = view;

    let mut terminal = setup::setup_terminal()?;
    run::run(&mut app, &mut terminal)?;
    setup::restore_terminal(&mut terminal)?;
    Ok(())
}
