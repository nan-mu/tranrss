#[cfg(test)]
use crate::Result as _Result;

#[tokio::test]
async fn test_get_rss() -> _Result<()> {
    //验证网络库是否可用
    use crate::get_text;
    use feed_rs::parser;
    let urls = vec!["https://rustcc.cn/rss", "https://rustmagazine.org/feed.xml"];
    let texts = get_text(urls);
    for content in texts.await? {
        let feed = parser::parse(&content[..])?;
        println!("{:?}", feed.title);
    }
    Ok(())
}
