use bytes;
use feed_rs::parser;
use log::{debug, info};
use log4rs;
use reqwest::get;
use std::error::Error;
use tokio::spawn;

mod test;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("start");
    let urls = vec![
        "https://rustcc.cn/rss",             //RSS 2.O
        "https://rustmagazine.org/feed.xml", //Atom
        "https://readrust.net/all/feed.rss", //RSS 1.O
    ];
    let texts = get_text(urls);
    for content in texts.await? {
        let feed = parser::parse(&content[..])?;
        info!("{:?}", feed.feed_type);
        // info!("{:?}", feed);
    }
    Ok(())
}

async fn get_text(urls: Vec<&'static str>) -> Result<Vec<bytes::Bytes>> {
    let mut res = Vec::new();
    let mut connects = Vec::new();
    for url in urls {
        debug!("访问 {} 开始", url);
        connects.push(spawn(get(url)));
        // res.push(get(url).await?.bytes().await?);
    }
    for connect in connects {
        debug!("访问结束");
        res.push(connect.await??.bytes().await?);
    }

    Ok(res)
}
