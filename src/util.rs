use std::collections::HashSet;
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio_stream::{self as stream, StreamExt};
use twilight_http::api_error::ApiError;
use twilight_http::error::ErrorType;
use twilight_model::id::Id;

use crate::bot::Context;
use crate::db::ChannelId;
use crate::error::Error;

pub type CliResult = std::result::Result<(), Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn is_unknown_channel_error(err: &ErrorType) -> bool {
    matches!(err,
        ErrorType::Response {
            error: ApiError::General(e),
            status,
            ..
        } if status.get() == 404 && e.code == 10003
    )
}

#[derive(Debug)]
pub struct ContentBuilder {
    limit: usize,
    pub content: Vec<String>,
}

impl ContentBuilder {
    pub fn new(limit: usize) -> Self {
        Self {
            content: vec![],
            limit,
        }
    }
}

impl Default for ContentBuilder {
    fn default() -> Self {
        Self::new(2000)
    }
}

impl IntoIterator for ContentBuilder {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}

impl fmt::Write for ContentBuilder {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.content.last_mut() {
            Some(current) => {
                if current.len() + s.len() > self.limit {
                    self.content.push(String::from(s));
                } else {
                    current.push_str(s);
                }
            }
            None => {
                self.content.push(String::from(s));
            }
        };
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        match self.content.last_mut() {
            Some(current) => {
                if current.len() + c.len_utf8() > self.limit {
                    self.content.push(c.to_string());
                } else {
                    current.push(c);
                }
            }
            None => self.content.push(c.to_string()),
        };
        Ok(())
    }
}

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn format_timestamp(seconds: i64) -> String {
    use time::format_description::FormatItem;
    use time::macros::format_description;
    use time::OffsetDateTime;

    const FMT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day] [hour]:[minute]");

    if let Ok(v) = OffsetDateTime::from_unix_timestamp(seconds) {
        if let Ok(s) = v.format(&FMT) {
            return s;
        }
    }
    String::new()
}

pub fn strip_html_tags<S>(input: S) -> String
where
    S: AsRef<str>,
{
    use kuchiki::traits::*;

    kuchiki::parse_html().one(input.as_ref()).text_contents()
}

#[cfg(test)]
mod tests {
    use super::ContentBuilder;
    use std::fmt::Write;

    #[test]
    fn content_builder() {
        let mut c = ContentBuilder::new(20);

        let _ = write!(&mut c, "{}", "foo".repeat(5));
        assert_eq!(c.content.len(), 1);

        let _ = write!(&mut c, "{}", "foo".repeat(5));
        assert_eq!(c.content.len(), 2);
        assert_eq!(c.content[0], "foo".repeat(5));
        assert_eq!(c.content[1], "foo".repeat(5));

        let _ = c.write_char('f');
        let _ = c.write_char('o');
        let _ = c.write_char('o');
        assert_eq!(c.content.len(), 2);
        assert_eq!(c.content[1], "foo".repeat(6));

        let _ = c.write_str("foobar");
        assert_eq!(c.content.len(), 3);
        assert_eq!(c.content[0], "foo".repeat(5));
        assert_eq!(c.content[1], "foo".repeat(6));
        assert_eq!(c.content[2], "foobar");
    }
}