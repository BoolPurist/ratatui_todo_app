use ratatui::layout::{Constraint, Direction, Layout, Rect};
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

        let splits = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(inner);
        let (space_todo_list, space_loading) = (splits[1], splits[0]);
        let todo_list = create_todo_list_block(ctx, space_todo_list);

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

fn create_todo_list_block(ctx: &AppContext, draw_area: Rect) -> Paragraph {
    let container = Block::default()
        .borders(Borders::ALL)
        .title("List")
        .padding(Padding {
            top: 1,
            bottom: 1,
            left: 2,
            right: 2,
        });

    let selection = ctx.selection;
    let list: Vec<Line> = ctx
        .todos
        .iter()
        .enumerate()
        .flat_map(|(index, todo)| drawing::draw_one_todo(todo, index, selection))
        .collect();

    let height_to_selection = calc_scroll(selection, draw_area);
    let list = Paragraph::new(Text::from(list))
        .wrap(Wrap { trim: true })
        .block(container)
        .scroll((height_to_selection as u16, 0));

    list
}

fn calc_scroll(selection: Selection, draw_area: Rect) -> usize {
    const NO_SCROLL_Y: usize = 0;
    if let Some(index) = selection {
        let index = index as usize;
        let height_to_selection = (constants::LINE_SPACE_BETWEEN_TODOS + 1) * index;
        let threshold = (draw_area.height / 2) as usize;

        if (threshold) > height_to_selection {
            NO_SCROLL_Y
        } else {
            height_to_selection - threshold
        }
    } else {
        NO_SCROLL_Y
    }
}
