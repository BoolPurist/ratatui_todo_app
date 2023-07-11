use crate::{
    input::{self, AppInput},
    prelude::*,
    AppContext,
};

pub fn run(app: &mut AppContext, terminal: &mut AppBackEndTerminal) -> AppResult<()> {
    loop {
        let event = input::handle_input()?;
        if let AppInput::Quit = event {
            break;
        }

        app.update(&event);
        app.render(terminal)?;
    }
    Ok(())
}
