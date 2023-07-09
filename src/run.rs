use crate::{
    input::{self, AppInput},
    prelude::*,
    AppContext,
};

pub fn run(app: &mut AppContext, terminal: &mut AppBackEndTerminal) -> AppResult<()> {
    loop {
        let event = input::handle_input()?;
        if AppInput::Quit == event {
            break;
        }
        app.render(terminal)?;
    }
    Ok(())
}
