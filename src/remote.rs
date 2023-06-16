use crate::model::Contest;

pub async fn pull_contest_data(url: &str) -> Result<Vec<Contest>, anyhow::Error>{
  let body = reqwest::get(url)
    .await?
    .text()
    .await?;
  let contests = serde_json::from_str::<Vec<Contest>>(&body)?;

  Ok(contests)
}