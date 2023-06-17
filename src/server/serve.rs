use std::collections::HashSet;

use askama::Template;
use time::{OffsetDateTime, macros::offset};

use crate::{db::{open_db, query_unend_contest}, model::{Contest, AtomContext}};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct QueryParam {
    day: Option<i64>,
    source: Option<String>,
}
impl QueryParam{
  fn apply_filter<T>(&self, iter: T)-> Vec<Contest> where T: Iterator<Item = Contest> {
    let days = self.day.unwrap_or(i64::MAX);
    let allow_src = if let Some(ref rules) = self.source {
      HashSet::from_iter(rules.split(',').map(str::trim))
    } else {
      HashSet::new()
    };

    let now_utc = OffsetDateTime::now_utc();

    iter
      .filter(|item| item.start_time.year() >= now_utc.year())
      .filter(|item| (item.start_time - now_utc).whole_days() < days)
      .filter(|item| allow_src.is_empty() || allow_src.contains(item.source.as_str()))
      .map(|item| item.to_offset(offset!(+8)))
      .collect()
  }
}

pub async fn json_handler(params: QueryParam) -> Result<impl warp::Reply, warp::Rejection> {
    // Ok(warp::reply::json(&params))

  let db = open_db();
  let data = query_unend_contest(&db).unwrap();
  let data = params.apply_filter(data.into_iter());
  Ok(warp::reply::json(&data))
}


pub async fn rss_handler(params: QueryParam) -> Result<impl warp::Reply, warp::Rejection>{
  let db = open_db();
  let data = query_unend_contest(&db).unwrap();
  let data = params.apply_filter(data.into_iter());


  let atom = AtomContext::new(data, "".to_owned());
  let content = atom.render();
  Ok(warp::reply::with_header(content.unwrap(), "Content-Type", "text/xml"))
}