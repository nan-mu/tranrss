use bytes;
use feed_rs::{model, parser};
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
    //任务路径：
    //1. 从订阅源提取文章。
    //2. 提取需要翻译的段落。
    //  a. 有的文章在content中，有的在summary中，需要逻辑判断一下。
    //3. 在内存中组合翻译内容和原文。
    //4. 输出到文件，拼合链接。
    let texts = get_text(urls);
    let mut feeds = Vec::new();
    for content in texts.await? {
        feeds.push(parser::parse(&content[..])?);
        info!("{:?}", feeds.last().unwrap().feed_type);
        extract_articles_title(&feeds.last().unwrap()).await?;
    }

    debug!("end");
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

async fn extract_articles_title(feed: &model::Feed) -> Result<()> {
    //从订阅源提取文章
    for entry in &feed.entries {
        debug!("文章标题：{}", entry.title.as_ref().unwrap().content);
        match entry.content.as_ref() {
            Some(content) => {
                debug!("文章内容：{}", content.body.as_ref().unwrap());
            }
            None => {
                debug!("文章内容：{}", entry.summary.as_ref().unwrap().content);
            }
        }
    }
    Ok(())
}
