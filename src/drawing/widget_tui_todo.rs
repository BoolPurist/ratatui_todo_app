
use std::{borrow::Cow, cell::OnceCell};

use ratatui::{
    layout::Rect,
    style::{Color, Style, Modifier},
    text::Span,
    widgets::Widget,
};

use textwrap::{wrap, Options, WordSplitter};

use crate::Todo;

pub struct WidgetTodo<'a> {
    todo: &'a Todo,
    wrapped_text: OnceCell<Vec<Cow<'a, str>>>,
    checkmark: OnceCell<Span<'static>>,
    width_to_wrap: u16,
    is_selected: bool,
}

impl<'a> Widget for WidgetTodo<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let check_mark = self.create_checkmark();
        let (x, y) = (area.x, area.y);
        let check_mark_width = check_mark.width() as u16;

        let no_check_mark_poss = check_mark_width > area.width;
        if no_check_mark_poss {
            return;
        }
        
        if self.is_selected {
            buf.set_style(area, Style::default().add_modifier(Modifier::BOLD));
        }
        buf.set_span(x, y, check_mark, check_mark_width);

        let x_after_check_mark = x + check_mark_width;
        let mut y_rising = y;

        for next_line in self.wrapped() {
            buf.set_string(x_after_check_mark, y_rising, next_line, Style::default());
            y_rising += 1;
        }
    }
}

impl<'a> WidgetTodo<'a> {
    pub fn new(todo: &'a Todo, width_to_wrap: u16, is_selected: bool) -> Self {
        Self {
            todo,
            width_to_wrap,
            is_selected,
            wrapped_text: Default::default(),
            checkmark: Default::default(),
        }
    }

    pub fn required_height(&self) -> usize {
        self.wrapped().len()
    }

    fn width_after_checkmark(&self) -> usize {
        let checkmark = self.create_checkmark();
        (self.width_to_wrap as usize).saturating_sub(checkmark.width())  
    }

    fn wrapped(&self) -> &[Cow<'a, str>] {
        self.wrapped_text.get_or_init(|| {
            let width = self.width_after_checkmark();

            let options = Options::new(width).word_splitter(WordSplitter::NoHyphenation);
            wrap(self.todo.content().as_str(), options)
        })
    }

    #[rustfmt::skip]
    fn create_checkmark(&self) -> &Span<'static> {
        self.checkmark.get_or_init(|| {
            let done = self.todo.done();
            
            let style_depends_on_done = if done {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Red)
            };
            
            match (self.is_selected, self.todo.done()) {
                (true, true)   => Span::styled(">>   [x]   ", style_depends_on_done),
                (true, false)  => Span::styled(">>   [ ]   ", style_depends_on_done),
                (false, true)  => Span::styled("     [x]   ", style_depends_on_done),
                (false, false) => Span::styled("     [ ]   ", style_depends_on_done),
            }
        })
    }
}
