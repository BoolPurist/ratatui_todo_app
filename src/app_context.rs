use crate::Todo;

use crate::drawing;
use crate::prelude::*;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::text::Text;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;

pub struct AppContext {
    todos: Vec<Todo>,
}

impl AppContext {
    pub fn new(todos: impl IntoIterator<Item = Todo>) -> Self {
        Self {
            todos: todos.into_iter().collect(),
        }
    }

    fn calc_sides_horz(&self, size: Rect) -> (u16, u16) {
        let min_width = self
            .todos
            .iter()
            .map(|todo| todo.recommend_width())
            .max()
            .unwrap_or(5) as u16;
        let width = size.width;
        if width > min_width {
            let side = (size.width - min_width) / 2;
            (min_width, side)
        } else {
            (min_width, 0)
        }
    }

    pub fn render(&self, tui: &mut AppBackEndTerminal) -> AppResult<()> {
        use ratatui::layout::Constraint;
        use ratatui::layout::Direction;
        use ratatui::layout::Layout;
        use ratatui::widgets::Block;
        use ratatui::widgets::Borders;

        tui.draw(|frame| {
            let whole_size = frame.size();
            let container = Block::default().borders(Borders::ALL).title("Todo List");
            let inner_size = container.inner(whole_size);
            let (middle, side) = self.calc_sides_horz(inner_size);
            let (left, right) = (side, side);

            let laytout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(left),
                    Constraint::Length(middle),
                    Constraint::Length(right),
                ])
                .split(inner_size);
            let todo_container = Block::default()
                .borders(Borders::ALL)
                .padding(Padding::uniform(1));
            let list: Vec<Line> = { self.todos.iter().map(drawing::draw_one_todo).collect() };

            let list = Paragraph::new(Text::from(list))
                .wrap(Wrap { trim: true })
                .block(todo_container);

            frame.render_widget(container, whole_size);
            frame.render_widget(list, laytout[1]);
        })?;
        Ok(())
    }
}
