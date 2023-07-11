use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;

use crate::prelude::*;
use crate::AppContext;

pub fn render(ctx: &AppContext, tui: &mut AppBackEndTerminal) -> AppResult<()> {
    tui.draw(|frame| {
        let whole_size = frame.size();

        let block = Block::default()
            .borders(Borders::ALL)
            .title("New todo")
            .padding(Padding::uniform(1));

        let inner_size = block.inner(whole_size);
        let input_widget = ctx.creation_mask.widget();

        frame.render_widget(block, whole_size);
        if let Some(error_message) = ctx.submission_error.as_deref() {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
                .split(inner_size);
            let error_widget = Paragraph::new(error_message)
                .style(Style::default().fg(Color::Red))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Invalid content for label of a todo"),
                );

            frame.render_widget(error_widget, layout[0]);
            frame.render_widget(input_widget, layout[1]);
        } else {
            frame.render_widget(input_widget, inner_size);
        }
        // frame.render_widget(list, laytout[1]);
    })?;
    Ok(())
}
