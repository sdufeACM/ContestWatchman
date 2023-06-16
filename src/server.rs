use std::collections::HashSet;

use time::OffsetDateTime;
use warp::Filter;

use crate::db::{open_db, query_unend_contest};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct QueryParam {
    day: Option<i64>,
    source: Option<String>,
}

async fn handler(params: QueryParam) -> Result<impl warp::Reply, warp::Rejection> {
    // Ok(warp::reply::json(&params))
    dbg!(&params);
    let db = open_db();
    let allow = if let Some(all) = params.source {
      let keys = all.split(",").map(|v| v.trim().to_owned());
      HashSet::from_iter(keys)
    } else{
      HashSet::new()
    };
    let contest = query_unend_contest(&db)
        .unwrap()
        .into_iter()
        .filter(|item| {
            if let Some(day) = params.day {
                (item.start_time - OffsetDateTime::now_utc()).whole_days() <= day
            } else {
                true
            }
        })
        .filter(|item| {
          if allow.is_empty() {
            true
          }else{
            allow.contains(&item.source)
          }
        })
        .collect::<Vec<_>>();
    Ok(warp::reply::json(&contest))
}

pub async fn serve(port: u16) {
    let page_db = warp::get()
        .and(warp::path::end())
        .and(warp::query::<QueryParam>())
        .and_then(handler);

    warp::serve(page_db).run(([0, 0, 0, 0], port)).await;
}
