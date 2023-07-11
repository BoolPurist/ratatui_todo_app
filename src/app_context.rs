use crate::Todo;

use crate::drawing;
use crate::prelude::*;

use ratatui::text::Line;
use ratatui::text::Text;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;

pub struct AppContext {
    todos: Vec<Todo>,
    selection: Selection,
}

impl AppContext {
    pub fn new(todos: impl IntoIterator<Item = Todo>) -> Self {
        Self {
            todos: todos.into_iter().collect(),
            selection: None,
        }
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

    pub fn render(&self, tui: &mut AppBackEndTerminal) -> AppResult<()> {
        use ratatui::widgets::Block;
        use ratatui::widgets::Borders;

        tui.draw(|frame| {
            let whole_size = frame.size();
            let container = Block::default()
                .borders(Borders::ALL)
                .title("Todo List")
                .padding(Padding {
                    top: 1,
                    bottom: 1,
                    left: 2,
                    right: 2,
                });

            let list: Vec<Line> = self
                .todos
                .iter()
                .enumerate()
                .map(|(index, todo)| drawing::draw_one_todo(todo, index, self.selection))
                .flatten()
                .collect();

            let list = Paragraph::new(Text::from(list))
                .wrap(Wrap { trim: true })
                .block(container);

            frame.render_widget(list, whole_size);
            // frame.render_widget(list, laytout[1]);
        })?;
        Ok(())
    }
}
