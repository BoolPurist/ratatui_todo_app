use crate::{prelude::*, AppContext};
use std::collections::VecDeque;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Paragraph, Wrap, Block, Borders}, text::{Line, Span}, style::{Style, Color}, Frame,
};

use crate::Todo;

use super::WidgetTodo;

pub fn render_todos(
    tui: &mut AppBackEndTerminal,
    ctx: &AppContext,
) -> AppResult {
    
        tui.draw(|frame| {
            

        let size = {
            let block = Block::default().borders(Borders::ALL).title("Todo List");
            let size = block.inner(frame.size());
            frame.render_widget(block, frame.size());

            draw_info_and_errors(ctx, frame, size)
        };
        
        let whole_height = size.height;
        let width = size.width;
        

        let todos = &ctx.todos;
        if todos.is_empty() {
            return;
        }
         
        let current_selection = ctx.selection;
        let widgets_to_draw = filter_accroding_selection(todos, current_selection as usize, width, whole_height);
        if widgets_to_draw.is_empty() {
            let from_one = current_selection + 1;
            frame.render_widget(
                Paragraph::new(format!("Current Element {}.\nCurrent height of terminal window is too low to show any item.", from_one))
                    .wrap(Wrap { trim: false }),
                size,
            );
        }

        let layout = {
            let constriants: Vec<Constraint> = widgets_to_draw
                .iter()
                .map(|widget| Constraint::Min(widget.required_height() as u16))
                .collect();
            Layout::default()
                .direction(Direction::Vertical)
                .constraints(constriants)
                .split(size)
        };
        
        assert!(layout.len() == widgets_to_draw.len());

        for (to_draw, next_area) in widgets_to_draw.into_iter().zip(layout.iter()) {
            frame.render_widget(to_draw, *next_area);
            
        }
    })?;
    Ok(())
}

fn filter_accroding_selection(
    todos: &[Todo],
    current_selection: usize,
    width: u16,
    whole_height: u16,
) -> VecDeque<WidgetTodo<'_>> {
    let mut before_selection = todos.iter().take(current_selection).rev();
    let current_selected_widget =
        WidgetTodo::new(todos.get(current_selection).unwrap(), width, true);
    let mut after_selection = todos.iter().skip(current_selection.saturating_add(1));

    let mut widgets_to_draw = VecDeque::new();
    let mut current_height: u16 = current_selected_widget.required_height() as u16;

    if current_not_over_whole(current_height, whole_height) {
        widgets_to_draw.push_back(current_selected_widget);

        const NO_MORE_WIDGETS: bool = true;
        const ONE_MORE_WIDGET: bool = false;

        loop {
            let no_more_before = if let Some(next_before) = before_selection.next() {
                if let Some(to_add) =
                    enough_space_for(next_before, width, &mut current_height, whole_height)
                {
                    widgets_to_draw.push_front(to_add);
                    ONE_MORE_WIDGET
                } else {
                    NO_MORE_WIDGETS
                }
            } else {
                NO_MORE_WIDGETS
            };

            let no_more_after = if let Some(next_after) = after_selection.next() {
                if let Some(to_add) =
                    enough_space_for(next_after, width, &mut current_height, whole_height)
                {
                    widgets_to_draw.push_back(to_add);
                    ONE_MORE_WIDGET
                } else {
                    NO_MORE_WIDGETS
                }
            } else {
                NO_MORE_WIDGETS
            };

            if no_more_before && no_more_after {
                break;
            }
        }
    }

    return widgets_to_draw;

    fn enough_space_for<'a>(
        next: &'a Todo,
        width: u16,
        current_height: &mut u16,
        whole_height: u16,
    ) -> Option<WidgetTodo<'a>> {
        let next_widget = WidgetTodo::new(next, width, false);
        let height = next_widget.required_height() as u16;
        *current_height += height;
        if current_not_over_whole(*current_height, whole_height) {
            Some(next_widget)
        } else {
            None
        }
    }
}

fn current_not_over_whole(current_height: u16, whole_height: u16) -> bool {
    current_height < whole_height
}


fn draw_info_and_errors(ctx: &AppContext, tui: &mut Frame<AppBackEnd>, left_size: Rect) -> Rect {
    let mut lines:  Vec<Line> = Vec::new();
    
    for error in ctx.has_failed() {
        lines.push(Line::from(Span::styled(error.to_string(), Style::default().fg(Color::Red))));
    }
    if ctx.is_saving() {
        lines.push(Line::from(Span::styled("Saving Todos", Style::default().fg(Color::Blue))));
    }

    if !lines.is_empty() {
        
        let block = Block::default().borders(Borders::ALL);
        let spaces = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(left_size);
        let (space_info_box, space_left) = (spaces[0], spaces[1]);
        let text = Paragraph::new(lines).block(block);
        tui.render_widget(text, space_info_box);
        
        space_left
    } else {
        left_size
    }
    
    
}
