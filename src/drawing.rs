use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::Todo;
const TICK: &str = "- [x] \n";
pub fn draw_one_todo(todo: &Todo) -> Line {
    let done = todo.done();
    let (color, tick) = (
        if done { Color::Green } else { Color::Red },
        if done { TICK } else { "- [ ] \n" },
    );
    vec![
        Span::styled(tick, Style::default().fg(color)),
        Span::raw(todo.content().to_string()),
    ]
    .into()
}
