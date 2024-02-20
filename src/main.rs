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
        //RSS 2.O 有title description。
        "https://rustcc.cn/rss", //文章在entries有id title links summary(正文在这里)
        //Atom 有id title updated authors description link generator。
        "https://rustmagazine.org/feed.xml", //文章在entries有id title updated authors content(正文在这) links
        //RSS 2.O(理论上是1.1)。有title update description links
        "https://readrust.net/all/feed.rss", //文章在entries有id title updated authors summary(正文在这) links
    ];
    let texts = get_text(urls);
    for content in texts.await? {
        let feed = parser::parse(&content[..])?;
        info!("{:?}", feed.feed_type);
    }
    Ok(())
}

async fn get_text(urls: Vec<&'static str>) -> Result<Vec<bytes::Bytes>> {
    let mut res = Vec::new();
    let mut connects = Vec::new();
    for url in urls {
        debug!("访问 {} 开始", url);
        connects.push(spawn(get(url)));
    }
    for connect in connects {
        res.push(connect.await??.bytes().await?);
    }

    Ok(res)
}
