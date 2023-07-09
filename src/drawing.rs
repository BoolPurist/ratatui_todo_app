use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::Todo;

pub fn draw_one_todo(todo: &Todo) -> ListItem {
    let done = todo.done();
    let (color, tick) = (
        if done { Color::Green } else { Color::Red },
        if done { "- [x] " } else { "- [ ] " },
    );
    let line: Line = vec![
        Span::styled(tick, Style::default().fg(color)),
        Span::raw(todo.content().to_string()),
    ]
    .into();
    ListItem::new(line)
}
