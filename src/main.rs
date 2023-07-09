#[allow(unused_imports)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

pub use app_context::AppContext;
pub use todo::Todo;
pub use trimmed_text::TrimmedText;

mod app_context;
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
            Todo::dev_new("Hello"),
            Todo::dev_new("Hello xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx222222xx"),
        ]
        .into_iter(),
    );

    let mut terminal = setup::setup_terminal()?;
    run::run(&mut app, &mut terminal)?;
    setup::restore_terminal(&mut terminal)?;
    Ok(())
}
