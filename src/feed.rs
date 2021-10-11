use crate::db::{article::AddArticle, source::Source};
use chrono::DateTime;
use rss::Channel;

pub async fn get_channel_info(url: &str) -> Option<Channel> {
    let content = reqwest::get(url).await.ok()?.bytes().await.ok()?;
    let channel = Channel::read_from(&content[..]).ok()?;
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
        let itms = get_items(source).await;
        if let Some(itms) = itms {
            feed.extend(itms);
        }
    }
    feed.sort_by(|f1, f2| f1.pub_date.cmp(&f2.pub_date).reverse());
    Some(feed)
}
