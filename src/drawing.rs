pub mod draw_todo_create_mask;
pub mod draw_todo_list;

use crate::{prelude::*, Todo};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub fn draw_one_todo(todo: &Todo, index: usize, selection: Selection) -> Vec<Line> {
    let done = todo.done();
    let is_selected = if let Some(current_selection) = selection {
        current_selection == index as u16
    } else {
        false
    };

    let (color, tick) = (
        if done { Color::Green } else { Color::Red },
        match (done, is_selected) {
            (true, true) => constants::SELECTED_TICK,
            (true, false) => constants::TICK,
            (false, true) => constants::SELECTED_UNTICK,
            _ => constants::UNTICKED,
        },
    );

    let style = {
        let mut base_styple = Style::default().fg(color);
        if is_selected {
            base_styple = base_styple.add_modifier(Modifier::BOLD);
        }
        base_styple
    };

    let first_line: Line = vec![
        Span::styled(tick, style),
        Span::raw(todo.content().to_string()),
    ]
    .into();

    iter::once(first_line)
        .chain(iter::repeat(Line::from(String::new())).take(constants::LINE_SPACE_BETWEEN_TODOS))
        .collect()
}
