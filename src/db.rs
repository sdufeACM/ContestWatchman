use std::env;

use rusqlite::{Connection, ErrorCode, OptionalExtension};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::model::Contest;

static DATABASE_CODE: u32 = 0;

fn init_database(conn: &mut Connection) -> Result<(), anyhow::Error> {
    let transaction = conn.transaction()?;
    let version_code = transaction
        .query_row(
            "SELECT version_code FROM table_version WHERE id = 0",
            (),
            |row| Ok(row.get(0).unwrap()),
        )
        .optional()
        .ok()
        .flatten();
    if version_code.is_none() {
        transaction.execute(
            "CREATE TABLE table_version(
      id INTEGER PRIMARY KEY,
      version_code NUMBER
    )",
            [],
        )?;
        transaction.execute("INSERT INTO table_version VALUES(0, ?1)", [DATABASE_CODE])?;

        transaction.execute(
            "CREATE TABLE contest(
      hash_key TEXT PRIMARY KEY NOT NULL,
      source TEXT,
      name TEXT,
      start_time DATETIME,
      end_time DATETIME,
      link TEXT NULL
     )",
            (),
        )?;
    }
    let version_code = version_code.unwrap_or(DATABASE_CODE);
    if version_code < DATABASE_CODE {
        // upgrade table
        todo!()
    }

    transaction.commit().map_err(Into::<anyhow::Error>::into)
}

pub fn open_db() -> Connection {
    let db_path = env::var("WATCHMAN_DATABASE").unwrap_or_else(|_| String::from("contest.db"));
    let mut connection = Connection::open(db_path).unwrap();
    init_database(&mut connection).unwrap();
    connection
}

pub fn insert_db(conn: &Connection, item: &Contest) -> Result<bool, anyhow::Error> {
    let ret = conn.execute(
    "INSERT INTO contest(hash_key, source, name, start_time, end_time, link) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", 
    (item.id().to_string(), &item.source, &item.name, item.start_time.format(&Rfc3339).unwrap(), item.end_time.format(&Rfc3339).unwrap(), &item.link));
    if let Err(e) = ret {
        if let Some(ErrorCode::ConstraintViolation) = e.sqlite_error_code() {
            Ok(false)
        } else {
            Err(e.into())
        }
    } else {
        println!("Insert: {:?}", item);
        Ok(true)
    }
}

pub fn query_unend_contest(conn: &Connection) -> Result<Vec<Contest>, anyhow::Error> {
    let cur = OffsetDateTime::now_utc();
    let values = conn
        .prepare(
            "SELECT source, name, start_time, end_time, link FROM contest WHERE end_time >= ?1",
        )?
        .query_map([cur.format(&Rfc3339).unwrap()], |row| {
            let start_time = row.get::<&str, String>("start_time").unwrap();
            let end_time = row.get::<&str, String>("end_time").unwrap();
            Ok(Contest {
                source: row.get("source").unwrap(),
                name: row.get("name").unwrap(),
                link: row.get("link").ok(),
                start_time: OffsetDateTime::parse(&start_time, &Rfc3339).unwrap(),
                end_time:  OffsetDateTime::parse(&end_time, &Rfc3339).unwrap(),
            })
        })?
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();

    Ok(values)
}
