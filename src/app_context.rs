pub use current_view::CurrentView;
pub use io_task_result::IoTaskResult;

mod current_view;
mod io_task_result;

use tui_textarea::TextArea;

use crate::{data_file_source, Todo};

use crate::drawing::{draw_todo_create_mask, draw_todo_list};
use crate::input::AppInput;
use crate::prelude::*;

#[derive(Default)]
pub struct AppContext {
    pub todos: Todos,
    pub selection: Selection,
    pub current_view: CurrentView,
    pub creation_mask: TextArea<'static>,
    pub submission_error: Option<String>,
    pending_save: IoTaskResult<()>,
}

impl Drop for AppContext {
    fn drop(&mut self) {
        if let IoTaskResult::Working(wait_on) = &mut self.pending_save {
            _ = wait_on.block_until_ready();
        }
    }
}

impl AppContext {
    pub fn new(todos: impl IntoIterator<Item = Todo>) -> Self {
        Self {
            todos: todos.into_iter().collect(),
            selection: None,
            current_view: Default::default(),
            creation_mask: Default::default(),
            pending_save: IoTaskResult::Idle,
            submission_error: None,
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

    pub fn is_saving(&self) -> bool {
        self.pending_save.is_working()
    }

    pub fn has_failed(&self) -> Vec<&AppError> {
        let mut errors = Vec::new();

        if let IoTaskResult::Failed(error) = &self.pending_save {
            errors.push(error);
        }

        errors
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
            CurrentView::TodoList => self.update_todo_list(event),
            CurrentView::TodoCreation => self.update_adding_todo(event),
        };

        self.pending_save.try_fetch_from_io_task();

        Ok(())
    }

    pub fn render(&self, tui: &mut AppBackEndTerminal) -> AppResult<()> {
        match self.current_view {
            CurrentView::TodoList => draw_todo_list::render(self, tui),
            CurrentView::TodoCreation => draw_todo_create_mask::render(self, tui),
        }
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

    fn update_adding_todo(&mut self, event: &AppInput) {
        match *event {
            AppInput::UserPressedEnter => {
                self.handle_submission();
            }
            AppInput::KeyEvent(key_event) => {
                if constants::DEFAULT_ESC == key_event.code {
                    self.submission_error = None;
                    self.creation_mask = TextArea::default();
                    self.current_view = CurrentView::TodoList;
                } else {
                    self.creation_mask.input(key_event);
                }
            }
            _ => (),
        }
    }

    fn update_todo_list(&mut self, event: &AppInput) {
        match event {
            AppInput::UserPresedUp => self.selection_up(),
            AppInput::UserPressedDown => self.selection_down(),
            AppInput::UserPressedEnter => self.toggle_todo(),
            AppInput::KeyEvent(key) => match key.code {
                constants::DEFAULT_ADD => self.current_view = CurrentView::TodoCreation,
                constants::DEFAULT_DELTE => self.delete_todo(),
                constants::DEFAULT_SAVE => self.save_if_not_pending(),
                _ => (),
            },

            _ => (),
        }
    }

    fn save_if_not_pending(&mut self) {
        if !self.pending_save.is_working() {
            let data = self.todos.clone();

            let promise = poll_promise::Promise::spawn_thread("Task: Saving Todos", move || {
                data_file_source::save_data(&data)
            });

            self.pending_save = IoTaskResult::Working(promise);
        }
    }

    fn delete_todo(&mut self) {
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
}
