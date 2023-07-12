use ratatui::layout::{Constraint, Direction, Layout};
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

        if ctx.is_saving() {
            let todo_list = create_todo_list_block(ctx);
            let splits = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
                .split(inner);
            let (space_todo_list, space_loading) = (splits[1], splits[0]);

            frame.render_widget(Paragraph::new("Saving Todos"), space_loading);
            frame.render_widget(todo_list, space_todo_list);
        } else {
            let todo_list = create_todo_list_block(ctx);

            frame.render_widget(todo_list, inner);
        }
    })?;
    Ok(())
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
