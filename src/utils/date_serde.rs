use chrono::{DateTime, NaiveDate};
use serde::{Deserializer, Serializer, de};

pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.format("%Y-%m-%d").to_string())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = <&str as de::Deserialize<'de>>::deserialize(deserializer)?;

    if s.contains('T') {
        let dt = DateTime::parse_from_rfc3339(s).map_err(de::Error::custom)?;
        Ok(dt.date_naive())
    } else {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(de::Error::custom)
    }
}
