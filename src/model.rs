use serde::{
    Deserialize, Serialize,
};
use time::{OffsetDateTime, Duration};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::Sub,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contest {
    pub source: String,
    pub name: String,
    pub link: Option<String>,
    #[serde(with="time::serde::iso8601")]
    pub start_time: OffsetDateTime,
    #[serde(with="time::serde::iso8601")]
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
}

// impl<'de> Deserialize<'de> for SystemTimeDef {
//     fn deserialize<D>(deserializer: D) -> Result<SystemTimeDef, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct SystemTimeVisitor;
//         impl<'de> Visitor<'de> for SystemTimeVisitor {
//             type Value = SystemTimeDef;
//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("str")
//             }
//             fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//             where
//                 E: de::Error,
//             {
//                 Ok(SystemTimeDef(v.to_owned()))
//             }
//         }
//         deserializer.deserialize_str(SystemTimeVisitor)
//     }
// }

// impl<'de> Deserialize<'de> for Contest {
//     fn deserialize<D>(deserializer: D) -> Result<Contest, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         enum Field {
//             Source,
//             Link,
//             Name,
//             StartTime,
//             EndTime,
//             Other,
//         }

//         impl<'de> Deserialize<'de> for Field {
//             fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
//             where
//                 D: Deserializer<'de>,
//             {
//                 struct FieldVisitor;

//                 impl<'de> Visitor<'de> for FieldVisitor {
//                     type Value = Field;

//                     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                         formatter.write_str("source, link, name, start_time or end_time")
//                     }
//                     fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//                     where
//                         E: de::Error,
//                     {
//                         match v {
//                             "source" => Ok(Field::Source),
//                             "link" => Ok(Field::Link),
//                             "name" => Ok(Field::Name),
//                             "start_time" => Ok(Field::StartTime),
//                             "end_time" => Ok(Field::EndTime),
//                             _ => Ok(Field::Other),
//                         }
//                     }
//                 }

//                 deserializer.deserialize_identifier(FieldVisitor)
//             }
//         }

//         struct ContestVisitor;
//         impl<'de> Visitor<'de> for ContestVisitor {
//             type Value = Contest;

//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("struct Contest")
//             }

//             fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//             where
//                 A: serde::de::MapAccess<'de>,
//             {
//                 let mut source = None;
//                 let mut name = None;
//                 let mut link = None;
//                 let mut start_time = None;
//                 let mut end_time = None;

//                 while let Some(key) = map.next_key()? {
//                     match key {
//                       Field::Name => name = Some(map.next_value()?),
//                       Field::Link => link = Some(map.next_value()?),
//                       Field::Source => source = Some(map.next_value()?),
//                       Field::StartTime => start_time = Some(map.next_value()?),
//                       Field::EndTime => end_time = Some(map.next_value()?),
//                       _ => {}
//                     }
//                 }

//                 let source = source.ok_or_else(|| de::Error::missing_field("source"))?;
//                 let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
//                 let start_time: String =
//                     start_time.ok_or_else(|| de::Error::missing_field("start_time"))?;
//                 let end_time: String =
//                     end_time.ok_or_else(|| de::Error::missing_field("end_time"))?;

//                 let start_time = start_time.parse::<DateTime<Utc>>().unwrap();
//                 let end_time = end_time.parse::<DateTime<Utc>>().unwrap();

//                 Ok(Contest {
//                     source,
//                     name,
//                     link,
//                     start_time: start_time.into(),
//                     end_time: end_time.into(),
//                 })
//             }
//         }

//         const FIELDS: &'static [&'static str] = &["source", "name", "link", "start_time", "end_time"];
//         deserializer.deserialize_struct("Contest", FIELDS,  ContestVisitor)
//     }
// }
