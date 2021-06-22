use crate::db::{article::AddArticle, source::Source};
use actix_web::{
    client::Client,
    web::{Buf, Bytes},
};
use chrono::DateTime;
use rss::Channel;

pub async fn fetch_rss_xml(url: &String) -> Option<Bytes> {
    let client = Client::default();
    let response = client
        .get(url)
        .header("User-Agent", "actix-web/3.0")
        .send()
        .await;
    response.ok()?.body().await.ok()
}

pub async fn get_channel_info(url: &String) -> Option<Channel> {
    let buf = fetch_rss_xml(url).await?;
    let channel = Channel::read_from(buf.bytes()).ok()?;
    Some(channel)
}

pub async fn get_items(source: Source) -> Option<Vec<AddArticle>> {
    let items = get_channel_info(&source.feed_link).await?;
    let articles = items.items.iter().map(|i| AddArticle {
        source_id: source.source_id,
        item_link: i.link.clone(),
        title: i.title.clone(),
        author: i.author.clone(),
        pub_date: i
            .pub_date
            .clone()
            .map(|t| DateTime::parse_from_rfc2822(t.as_ref()).unwrap()),
        description: i.description.clone(),
        content: i.content.clone(),
        guid: i.guid.clone().map(|g| g.value),
    });
    Some(articles.collect())
}

pub async fn generate_feed(sources: Vec<Source>) -> Option<Vec<AddArticle>> {
    let mut feed: Vec<AddArticle> = vec![];
    for source in sources {
        feed.extend(get_items(source).await?);
    }
    feed.sort_by(|f1, f2| f1.pub_date.cmp(&f2.pub_date).reverse());
    Some(feed)
}
