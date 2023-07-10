use crate::TrimmedText;

#[derive(Debug)]
pub struct Todo {
    content: TrimmedText,
    done: bool,
}

impl Todo {
    pub fn new(content: TrimmedText) -> Self {
        Self {
            content,
            done: false,
        }
    }

    pub fn dev_new(content: &str) -> Self {
        let trimmed = TrimmedText::new(content.to_string()).unwrap();
        Self::new(trimmed)
    }

    pub fn recommend_width(&self) -> usize {
        "- [ ] ".len() + self.content().len() + 10
    }

    pub fn mark_it_done(mut self) -> Self {
        self.done = true;
        self
    }

    pub fn content(&self) -> &TrimmedText {
        &self.content
    }

    pub fn done(&self) -> bool {
        self.done
    }
}
