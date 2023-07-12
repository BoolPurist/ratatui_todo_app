use std::time::Duration;

use crate::prelude::*;
use crossterm::event::{self, Event, KeyEvent};

#[derive(Debug, PartialEq, Eq)]
pub enum AppInput {
    Event(Event),
    KeyEvent(KeyEvent),
    UserPresedUp,
    UserPressedEnter,
    UserPressedDown,
    Tick,
    Quit,
}

pub fn handle_input() -> AppResult<AppInput> {
    if event::poll(Duration::from_millis(250))? {
        match event::read()? {
            Event::Key(key) => match key.code {
                constants::DEFAULT_QUIT => Ok(AppInput::Quit),
                constants::DEFAULT_ENTER => Ok(AppInput::UserPressedEnter),
                constants::DEFAULT_DOWN => Ok(AppInput::UserPressedDown),
                constants::DEFAULT_UP => Ok(AppInput::UserPresedUp),
                _ => Ok(AppInput::KeyEvent(key)),
            },
            event => Ok(AppInput::Event(event)),
        }
    } else {
        Ok(AppInput::Tick)
    }
}
