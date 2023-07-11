use std::time::Duration;

use crate::prelude::*;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

#[derive(Debug, PartialEq, Eq)]
pub enum AppInput {
    Event(Event),
    KeyEvent(KeyEvent),
    UserPresedUp,
    UserPressedEnter,
    UserPressedDown,
    GoBack,
    Tick,
    Quit,
}

pub fn handle_input() -> AppResult<AppInput> {
    if event::poll(Duration::from_millis(250))? {
        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => Ok(AppInput::Quit),
                KeyCode::Enter => Ok(AppInput::UserPressedEnter),
                KeyCode::Down => Ok(AppInput::UserPressedDown),
                KeyCode::Up => Ok(AppInput::UserPresedUp),
                KeyCode::Esc => Ok(AppInput::GoBack),
                _ => Ok(AppInput::KeyEvent(key)),
            },
            event => Ok(AppInput::Event(event)),
        }
    } else {
        Ok(AppInput::Tick)
    }
}
