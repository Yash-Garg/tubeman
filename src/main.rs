mod config;
mod feed;

use std::error::Error;

use futures::future::join_all;

use actix_web::{get, web, App, HttpServer, Responder};
use atom_syndication::Feed;
use config::Config;
use feed::YtFeed;

#[get("/")]
async fn index() -> impl Responder {
    "YT API // Alive"
}

#[get("/update")]
async fn update_feeds() -> impl Responder {
    let config = Config::load();

    let fetches = config
        .channels
        .iter()
        .flat_map(|(_, cids)| cids.iter().map(|cid| fetch_feed(cid)));

    let results = join_all(fetches).await;

    let feeds: Vec<YtFeed> = results
        .into_iter()
        .filter_map(|result| result.ok())
        .map(YtFeed::from)
        .collect();

    web::Json(feeds)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(update_feeds))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn fetch_feed(channel_id: &str) -> Result<Feed, Box<dyn Error>> {
    println!("Fetching feed for channel: {}", channel_id);

    let time = std::time::Instant::now();

    let content = reqwest::get(format!(
        "https://www.youtube.com/feeds/videos.xml?channel_id={}",
        channel_id
    ))
    .await?
    .bytes()
    .await?;

    println!("Fetched feed in: {:?}", time.elapsed());

    let feed = Feed::read_from(&content[..])?;

    Ok(feed)
}
