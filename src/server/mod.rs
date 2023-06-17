mod cmdlet;
mod serve;

use warp::Filter;

use self::{serve::{QueryParam, json_handler, rss_handler}, cmdlet::{pull_data_handler, PullParams}};





pub async fn serve(port: u16) {
    let page_db = warp::get()
        .and(warp::path("data"))
        .and(warp::path::end())
        .and(warp::query::<QueryParam>())
        .and_then(json_handler);

      let page_rss = warp::get()
        .and(warp::path("atom.xml"))
        .and(warp::path::end())
        .and(warp::query::<QueryParam>())
        .and_then(rss_handler);

      let cmdlet_pull = warp::get()
        .and(warp::path("cmdlet"))
        .and(warp::path("pull"))
        .and(warp::query::<PullParams>())
        .and_then(pull_data_handler);

      let routes = warp::get().and(
        page_db.or(page_rss)
          .or(cmdlet_pull)
      );

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}