use crate::TrimmedText;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
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

    pub fn mark_it_done(mut self) -> Self {
        self.done = true;
        self
    }

    pub fn content(&self) -> &TrimmedText {
        &self.content
    }
    pub fn toggle_done(&mut self) -> &mut Self {
        self.done = !self.done;
        self
    }

    pub fn done(&self) -> bool {
        self.done
    }
}

pub fn generate_random_of(number: u16, range: std::ops::Range<u16>) -> Vec<Todo> {
    let mut rng = rand::thread_rng();
    (1..=number)
        .map(|number| {
            // Exclusive range
            let n: u16 = rng.gen_range(range.clone());
            let lispsum_text = lipsum::lipsum(n as usize);
            let text: TrimmedText = format!("{}. {}", number, lispsum_text).try_into().unwrap();
            let done: bool = rand::random();
            if done {
                Todo::new(text).mark_it_done()
            } else {
                Todo::new(text)
            }
        })
        .collect()
}
