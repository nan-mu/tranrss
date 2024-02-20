use async_std::task;
use bytes;
use reqwest::get;
use rss::Channel;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //https://rustcc.cn/rss
    //https://rustmagazine.org/feed.xml
    let urls = vec!["https://rustcc.cn/rss", "https://rustmagazine.org/feed.xml"];
    let texts = task::block_on(get_text(urls))?;
    for content in texts {
        let channel = Channel::read_from(&content[..])?;
        println!("{:?}", channel);
    }
    Ok(())
}

async fn get_text(urls: Vec<&str>) -> Result<Vec<bytes::Bytes>, Box<dyn Error>> {
    let mut res = Vec::new();
    for url in urls {
        res.push(get(url).await?.bytes().await?);
    }
    Ok(res)
}
