use crate::{
    input::{self, AppInput},
    prelude::*,
    AppContext,
};

pub fn run(app: &mut AppContext, terminal: &mut AppBackEndTerminal) -> AppResult<()> {
    loop {
        let event = input::handle_input()?;
        match event {
            AppInput::Quit => break,
            AppInput::UserPresedUp => app.selection_up(),
            AppInput::UserPressedDown => app.selection_down(),
            _ => (),
        }
        app.render(terminal)?;
    }
    Ok(())
}
