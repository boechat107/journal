pub mod local_date {
    use chrono::prelude::{Date, DateTime, Local};
    use serde::de;
    use serde::Serializer;
    use std::fmt;

    pub fn serialize<S>(date: &Date<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.and_hms(0, 0, 0).to_string())
    }

    struct StrLocalDateVisitor;

    pub fn deserialize<'de, D>(d: D) -> Result<Date<Local>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(d.deserialize_str(StrLocalDateVisitor)?)
    }

    impl<'de> de::Visitor<'de> for StrLocalDateVisitor {
        type Value = Date<Local>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a string with chrono's default format")
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // The date is serialized as a DateTime. We need to get a Date.
            match value.parse::<DateTime<Local>>() {
                Ok(dt) => Ok(dt.date()),
                Err(e) => Err(E::custom(e)),
            }
        }
    }
}

pub mod local_datetime {
    use chrono::prelude::{DateTime, Local};
    use serde::de;
    use serde::Serializer;
    use std::fmt;

    pub fn serialize<S>(dt: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&dt.to_string())
    }

    struct StrLocalDateTimeVisitor;

    pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Local>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(d.deserialize_str(StrLocalDateTimeVisitor)?)
    }

    impl<'de> de::Visitor<'de> for StrLocalDateTimeVisitor {
        type Value = DateTime<Local>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a string with chrono's default format")
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match value.parse::<DateTime<Local>>() {
                Ok(dt) => Ok(dt),
                Err(e) => Err(E::custom(e)),
            }
        }
    }
}
