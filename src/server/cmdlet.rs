use crate::{db, remote};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PullParams{
  src: Option<String>
}
pub async fn pull_data_handler(params: PullParams)  -> Result<impl warp::Reply, warp::Rejection> {
    let connection = db::open_db();
    let src = params.src
        .as_ref()
        .map(String::as_str)
        .unwrap_or("https://contests.sdutacm.cn/contests.json")
        .to_owned();
    let data = remote::pull_contest_data(&src).await.unwrap();
    let mut buff = String::new();
    buff.push_str("<ul>");
    for item in data {
        if db::insert_db(&connection, &item).unwrap() {
          buff.push_str("<li>");
          buff.push_str(&format!("{:?}", item));
          buff.push_str("</li>");
        }
    }
    buff.push_str("</ul>");
    buff.push_str("<span>Task finish</span>");
    Ok(warp::reply::html(buff))
}
