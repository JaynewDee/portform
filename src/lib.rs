use chrono::prelude::*;
pub struct FormattedDate(String);

impl From<DateTime<Utc>> for FormattedDate {
    fn from(value: DateTime<Utc>) -> Self {
        let from_utc_now = value
            .to_string()
            .chars()
            .map(|c| match c {
                ':' | ' ' => '-',
                _ => c,
            })
            .collect();

        Self(from_utc_now)
    }
}
