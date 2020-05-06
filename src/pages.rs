use chrono::prelude::{Date, DateTime, Local};

#[derive(Debug)]
pub struct Page {
    id: u32,
    text: String,
    date: Date<Local>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    tags: Vec<String>,
}

impl Page {
    pub fn new(id: u32, text: String) -> Page {
        let now = Local::now();
        Page {
            id,
            text,
            date: Local::today(),
            created_at: now,
            updated_at: now,
            tags: vec![],
        }
    }
}
