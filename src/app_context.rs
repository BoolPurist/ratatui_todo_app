use tui_textarea::TextArea;

use crate::{data_file_source, Todo};

use crate::drawing::{draw_todo_create_mask, draw_todo_list};
use crate::input::AppInput;
use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurrentView {
    TodoList,
    TodoCreation,
}

impl TryFrom<String> for CurrentView {
    type Error = AppError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "TodoList" => Ok(Self::TodoList),
            "TodoAdd" => Ok(Self::TodoCreation),
            _ => Err(anyhow!(
                "Word \"{}\" is not a valid keyword for a start view",
                value
            )),
        }
    }
}

impl Default for CurrentView {
    fn default() -> Self {
        Self::TodoList
    }
}

#[derive(Default)]
pub struct AppContext {
    pub todos: Todos,
    pub selection: Selection,
    pub current_view: CurrentView,
    pub creation_mask: TextArea<'static>,
    pub submission_error: Option<String>,
}

impl AppContext {
    pub fn new(todos: impl IntoIterator<Item = Todo>) -> Self {
        Self {
            todos: todos.into_iter().collect(),
            selection: None,
            ..Default::default()
        }
    }

    pub fn view(&mut self, new_view: CurrentView) -> &mut Self {
        self.current_view = new_view;
        self
    }

    pub fn toggle_todo(&mut self) {
        if let Some(selection) = self.selection {
            self.todos
                .get_mut(selection as usize)
                .unwrap()
                .toggle_done();
        }
    }

    pub fn selection_up(&mut self) {
        let new_selection: Selection = match self.selection {
            None if !self.todos.is_empty() => Some(0),
            Some(selection) => Some(selection.saturating_sub(1)),
            _ => None,
        };
        self.selection = new_selection;
    }

    pub fn selection_down(&mut self) {
        let new_selection: Selection = match self.selection {
            None if !self.todos.is_empty() => Some(0),
            Some(selection) => Some(selection.saturating_add(1)),
            _ => None,
        }
        .map(|to_clamp| to_clamp.min(self.todos.len().min(TermNum::MAX as usize) as TermNum - 1));
        self.selection = new_selection;
    }

    pub fn update(&mut self, event: &AppInput) -> AppResult {
        match self.current_view {
            CurrentView::TodoList => match event {
                AppInput::UserPresedUp => self.selection_up(),
                AppInput::UserPressedDown => self.selection_down(),
                AppInput::UserPressedEnter => self.toggle_todo(),
                AppInput::KeyEvent(key) => match key.code {
                    constants::DEFAULT_ADD => self.current_view = CurrentView::TodoCreation,
                    constants::DEFAULT_DELTE => {
                        if let Some(current_selection) = self.selection {
                            let index = current_selection as usize;

                            self.todos.remove(index);
                            self.selection = if self.todos.is_empty() {
                                None
                            } else {
                                self.selection.map(|old| old.saturating_sub(1))
                            };
                        }
                    }
                    constants::DEFAULT_SAVE => {
                        data_file_source::save_data(&self.todos)?;
                    }
                    _ => (),
                },
                _ => (),
            },
            CurrentView::TodoCreation => match *event {
                AppInput::GoBack => {
                    self.submission_error = None;
                    self.creation_mask = TextArea::default();
                    self.current_view = CurrentView::TodoList;
                }
                AppInput::UserPressedEnter => {
                    self.handle_submission();
                }
                AppInput::KeyEvent(key_event) => {
                    self.creation_mask.input(key_event);
                }
                _ => (),
            },
        };

        Ok(())
    }

    fn handle_submission(&mut self) {
        let text_area = std::mem::take(&mut self.creation_mask);
        let content = text_area.into_lines();
        match content.join("\n").try_into() {
            Ok(trimmed_content) => {
                let new_todo = Todo::new(trimmed_content);
                self.todos.push(new_todo);
                self.current_view = CurrentView::TodoList;
            }
            Err(submission_error) => self.submission_error = Some(submission_error.to_string()),
        }
    }

    pub fn render(&self, tui: &mut AppBackEndTerminal) -> AppResult<()> {
        match self.current_view {
            CurrentView::TodoList => draw_todo_list::render(self, tui),
            CurrentView::TodoCreation => draw_todo_create_mask::render(self, tui),
        }
    }
}
