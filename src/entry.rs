use std::{fmt::Display, time::Instant};

use chrono::{DateTime, Local, SecondsFormat, TimeZone, Utc};

pub struct Entry {
    next: Option<Instant>,
    instant: Instant,
    datetime: String,
    duration: String,
    entry: String,
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_fmt(fmt)
        f.write_str(self.datetime.as_str())?;
        f.write_str("\t")?;
        f.write_str(self.duration.as_str())?;
        f.write_str("\t")?;
        f.write_str(self.entry.as_str())?;
        Ok(())
    }
}

impl Entry {
    pub fn utc() -> Self {
        Self::with_datetime(Instant::now(), Utc::now())
    }

    pub fn local() -> Self {
        Self::with_datetime(Instant::now(), Local::now())
    }

    pub fn before(&mut self, next: &Entry) -> &mut Self {
        self.next = Some(next.instant.clone());

        let duration = next.instant.duration_since(self.instant.clone());
        let seconds = duration.as_secs_f32();

        let display = if seconds > 60.0 {
            let minutes = (seconds / 60.0).round() as usize;
            format!("{}m", minutes)
        } else {
            let seconds = seconds.round() as usize;
            format!("{}s", seconds)
        };

        self.duration = display;

        self
    }

    pub fn push(&mut self, ch: char) {
        if ch == '\r' || ch == '\n' {
            return;
        }

        self.entry.push(ch);
    }

    pub fn backspace(&mut self) {
        if self.entry.len() == 0 {
            return;
        }

        self.entry.truncate(self.entry.len() - 1);
    }

    fn with_datetime<Tz>(instant: Instant, datetime: DateTime<Tz>) -> Self
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        let datetime = datetime.to_rfc3339_opts(SecondsFormat::Secs, true);

        Self {
            instant,
            datetime,
            entry: "".to_string(),
            duration: "--".to_string(),
            next: None,
        }
    }
}
