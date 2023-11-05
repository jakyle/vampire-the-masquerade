use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serializer};

pub fn date_to_string<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date = DateTime::<Utc>::from_naive_utc_and_offset(*date, Utc);
    let s = date.to_rfc3339();
    serializer.serialize_str(&s)
}

pub fn string_to_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let date = DateTime::parse_from_rfc3339(&s)
        .map_err(serde::de::Error::custom)?
        .naive_utc();
    Ok(date)
}
