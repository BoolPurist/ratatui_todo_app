use crate::Todo;

use crate::drawing;
use crate::prelude::*;
use ratatui::style::Color;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Text;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;

pub struct AppContext {
    todos: Vec<Todo>,
    selection: Option<u16>,
}

impl AppContext {
    pub fn new(todos: impl IntoIterator<Item = Todo>) -> Self {
        Self {
            todos: todos.into_iter().collect(),
            selection: None,
        }
    }

    pub fn selection_up(&mut self) {
        let new_selection: Option<u16> = match self.selection {
            None if !self.todos.is_empty() => Some(0),
            Some(selection) => Some(selection.saturating_sub(1)),
            _ => None,
        };
        self.selection = new_selection;
    }
    pub fn selection_down(&mut self) {
        let new_selection: Option<u16> = match self.selection {
            None if !self.todos.is_empty() => Some(0),
            Some(selection) => Some(selection.saturating_add(1)),
            _ => None,
        }
        .map(|to_clamp| to_clamp.min(self.todos.len().min(u16::MAX as usize) as u16 - 1));
        self.selection = new_selection;
    }

    fn handle_selection(&self, list: &mut [Line]) {
        if let Some(index) = self.selection {
            let index = index as usize;
            list.iter_mut()
                .nth(index)
                .expect("Given index from user selection is out of bounds for todo list")
                .patch_style(Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC));
        }
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

            let mut list: Vec<Line> = self.todos.iter().map(drawing::draw_one_todo).collect();
            self.handle_selection(&mut list);

            let list = Paragraph::new(Text::from(list))
                .wrap(Wrap { trim: true })
                .block(container);

            frame.render_widget(list, whole_size);
            // frame.render_widget(list, laytout[1]);
        })?;
        Ok(())
    }
}
