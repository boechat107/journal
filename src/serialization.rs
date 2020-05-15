pub mod local_date {
    use chrono::prelude::{Date, Local};
    use serde::Serializer;

    pub fn serialize<S>(date: &Date<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.and_hms(0, 0, 0).to_string())
    }
}

pub mod local_datetime {
    use chrono::prelude::{DateTime, Local};
    use serde::Serializer;

    pub fn serialize<S>(dt: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&dt.to_string())
    }
}
