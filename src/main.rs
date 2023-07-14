#[allow(unused_imports)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

use std::{env::VarError, str::FromStr};

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
    let lipsum = load_env_variable_dev(constants::dev::LIPSUM, false)?;

    let data = if lipsum {
        let (number_of_words, minimum_words, maximum_words) = (
            load_env_variable_dev(constants::dev::LIPSUM_ITEMS, 50)?,
            load_env_variable_dev(constants::dev::LIPSUM_MIN_WORD, 10)?,
            load_env_variable_dev(constants::dev::LIPSUM_MAX_WORD, 100)?,
        );
        Ok(todo::generate_random_of(
            number_of_words,
            minimum_words..maximum_words,
        ))
    } else {
        data_file_source::provide_data()
    }?;

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

fn load_env_variable_dev<T>(key: &str, default_value: T) -> AppResult<T>
where
    T: FromStr,
{
    if cfg!(debug_assertions) {
        match std::env::var(key) {
            Ok(to_resolve) => to_resolve.parse::<T>().map_err(|_| {
                anyhow!(
                    "Value ({}) could not be parsed correctly for key ({})",
                    to_resolve,
                    key
                )
            }),
            Err(VarError::NotPresent) => Ok(default_value),
            Err(error) => Err(anyhow!("Invalid value:\n Details: {}", error)),
        }
    } else {
        Ok(default_value)
    }
}
