use bytes;
use reqwest::get;
use rss::Channel;
use std::error::Error;

mod test;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    //https://rustcc.cn/rss
    //https://rustmagazine.org/feed.xml
    let urls = vec!["https://rustcc.cn/rss", "https://rustmagazine.org/feed.xml"];
    let texts = get_text(urls);
    for content in texts.await? {
        let channel = Channel::read_from(&content[..])?;
        println!("{:?}", channel);
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
