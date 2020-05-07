use chrono::prelude::{Date, DateTime, Local};
use std::collections::HashMap;

#[derive(Debug)]
struct Page {
    text: String,
    date: Date<Local>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    tags: Vec<String>,
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
        let now = Local::now();

        self.pages.insert(
            id,
            Page {
                text,
                date: Local::today(),
                created_at: now,
                updated_at: now,
                tags: vec![],
            },
        );
        self.id_cnt += 1;

        id
    }
}
