use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};

use crate::AppContext;
use crate::{drawing, prelude::*};

pub fn render(ctx: &AppContext, tui: &mut AppBackEndTerminal) -> AppResult<()> {
    tui.draw(|frame| {
        let whole_size = frame.size();
        let most_outer = Block::default()
            .borders(Borders::ALL)
            .title("Todos")
            .padding(Padding {
                top: 1,
                bottom: 1,
                left: 2,
                right: 2,
            });

        let inner = most_outer.inner(whole_size);
        frame.render_widget(most_outer, whole_size);

        let todo_list = create_todo_list_block(ctx);
        let splits = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(inner);
        let (space_todo_list, space_loading) = (splits[1], splits[0]);

        let info_box = create_info_box(ctx);
        frame.render_widget(info_box, space_loading);
        frame.render_widget(todo_list, space_todo_list);
    })?;

    return Ok(());

    fn create_info_box(ctx: &AppContext) -> Paragraph {
        let mut info_lines: Vec<Line<'static>> = Vec::new();

        if ctx.is_saving() {
            let mut next_line: Line = Line::from("Saving todos".to_string());
            next_line.patch_style(Style::default().fg(Color::Blue));
            info_lines.push(next_line);
        }

        for error in ctx.has_failed() {
            let mut next_line: Line = "Saving todos failed !".into();
            let mut second_line: Line = format!("Details: {}", error).into();
            let style = Style::default().fg(Color::Red);
            next_line.patch_style(style);
            second_line.patch_style(style);
            info_lines.push(next_line);
            info_lines.push(second_line);
        }

        Paragraph::new(info_lines)
    }
}

fn create_todo_list_block(ctx: &AppContext) -> Paragraph {
    let container = Block::default()
        .borders(Borders::ALL)
        .title("List")
        .padding(Padding {
            top: 1,
            bottom: 1,
            left: 2,
            right: 2,
        });

    let list: Vec<Line> = ctx
        .todos
        .iter()
        .enumerate()
        .flat_map(|(index, todo)| drawing::draw_one_todo(todo, index, ctx.selection))
        .collect();

    let list = Paragraph::new(Text::from(list))
        .wrap(Wrap { trim: true })
        .block(container);

    list
}
