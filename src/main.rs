use bytes;
use feed_rs::parser;
use log::info;
use log4rs;
use reqwest::get;
use std::error::Error;

mod test;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("start");
    let urls = vec!["https://rustcc.cn/rss", "https://rustmagazine.org/feed.xml"];
    let texts = get_text(urls);
    for content in texts.await? {
        let feed = parser::parse(&content[..])?;
        println!("{:?}", feed.title);
        // info!("{:?}", feed);
    }
    Ok(())
}

async fn get_text(urls: Vec<&str>) -> Result<Vec<bytes::Bytes>> {
    let mut res = Vec::new();
    for url in urls {
        res.push(get(url).await?.bytes().await?);
    }
    Ok(res)
}
