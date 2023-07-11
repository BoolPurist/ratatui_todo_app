use ratatui::text::{Line, Text};
use ratatui::widgets::{Padding, Paragraph, Wrap};

use crate::AppContext;
use crate::{drawing, prelude::*};

pub fn render(ctx: &AppContext, tui: &mut AppBackEndTerminal) -> AppResult<()> {
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

        let list: Vec<Line> = ctx
            .todos
            .iter()
            .enumerate()
            .flat_map(|(index, todo)| drawing::draw_one_todo(todo, index, ctx.selection))
            .collect();

        let list = Paragraph::new(Text::from(list))
            .wrap(Wrap { trim: true })
            .block(container);

        frame.render_widget(list, whole_size);
        // frame.render_widget(list, laytout[1]);
    })?;
    Ok(())
}
