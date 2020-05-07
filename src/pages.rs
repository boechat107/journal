use chrono::prelude::{Date, DateTime, Local};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Page {
    text: String,
    date: Date<Local>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    tags: Vec<String>,
}

impl Page {
    pub fn new(text: String) -> Page {
        let now = Local::now();
        Page {
            text,
            date: Local::today(),
            created_at: now,
            updated_at: now,
            tags: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Collection {
    pages: HashMap<u32, Page>,
    id_cnt: u32,
}

impl Collection {
    pub fn new() -> Collection {
        Collection {
            pages: HashMap::new(),
            id_cnt: 0,
        }
    }
    pub fn add(&mut self, text: String) -> u32 {
        let id = self.id_cnt;

        self.pages.insert(id, Page::new(text));
        self.id_cnt += 1;

        id
    }
}
