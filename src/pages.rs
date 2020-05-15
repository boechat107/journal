use crate::serialization::{local_date, local_datetime};
use chrono::prelude::{Date, DateTime, Local};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct Page {
    text: String,
    #[serde(serialize_with = "local_date::serialize")]
    date: Date<Local>,
    #[serde(serialize_with = "local_datetime::serialize")]
    created_at: DateTime<Local>,
    #[serde(serialize_with = "local_datetime::serialize")]
    updated_at: DateTime<Local>,
    tags: Vec<String>,
}

#[derive(Debug, Serialize)]
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

    pub fn len(&self) -> usize {
        self.pages.len()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_journal_pages() {
        let mut coll = Collection::new();

        let p1_id = coll.add(String::from("I've programmed in Rust"));
        let p2_id = coll.add(String::from("Code review"));

        assert_eq!(coll.len(), 2);

        let page1 = coll.pages.get(&p1_id).unwrap();
        assert_eq!(page1.created_at, page1.updated_at);

        let page2 = coll.pages.get(&p2_id).unwrap();
        assert_eq!(page2.created_at, page2.updated_at);

        assert_eq!(page1.date, page2.date);
    }
}
