type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::test]
async fn test_async_get_rss() -> Result<()> {
    use crate::get_text;
    use rss::Channel;

    let urls = vec!["https://rustcc.cn/rss", "https://rustmagazine.org/feed.xml"];
    let texts = get_text(urls).await?;
    for content in texts {
        let channel = Channel::read_from(&content[..]).expect("拆解返回正文失败");
        println!("{:?}", channel.title);
    }
    Ok(())
}
