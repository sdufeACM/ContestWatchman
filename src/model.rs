use askama::Template;
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::Sub,
};
use time::{Duration, OffsetDateTime, UtcOffset};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contest {
    pub source: String,
    pub name: String,
    pub link: Option<String>,
    #[serde(with = "time::serde::iso8601")]
    pub start_time: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end_time: OffsetDateTime,
}

impl Hash for Contest {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source.hash(state);
        self.name.hash(state);
        self.start_time.hash(state);
        self.link.hash(state);
        self.end_time.hash(state);
    }
}

impl Contest {
    pub fn duration(&self) -> Duration {
        if self.start_time > self.end_time {
            Duration::new(0, 0)
        } else {
            self.end_time.sub(self.start_time)
        }
    }

    pub fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn to_offset(mut self, offset: UtcOffset) -> Self{
        self.start_time = self.start_time.to_offset(offset);
        self.end_time = self.end_time.to_offset(offset);
        self
    }
}

#[derive(Template)]
#[template(path="atom.template.xml")]
pub struct AtomContext {
    entries: Vec<AtomEntry>,
    last_build: OffsetDateTime,
    url: String,
}

struct AtomEntry {
    id: u64,
    duration: f32,
    content: Contest,
}

impl From<Contest> for AtomEntry {
    fn from(v: Contest) -> Self {
        let duration = v.duration();
        let minutes = (duration.whole_minutes() % 60) as f32;
        let hours = duration.whole_hours() as f32;
        let duration = hours + minutes / 60.0;
        let duration = (duration * 10.0).round() / 10.0;

        Self {
            id: v.id(),
            duration: duration,
            content: v,
        }
    }
}

impl AtomContext {
    pub fn new(value: Vec<Contest>, url: String) -> Self {
        let last_build = value
            .iter()
            .map(|v| v.start_time)
            .min()
            .unwrap_or_else(|| OffsetDateTime::UNIX_EPOCH);

        let entries = value.into_iter().map(|v| v.into()).collect();

        Self {
            entries,
            last_build,
            url,
        }
    }
}
