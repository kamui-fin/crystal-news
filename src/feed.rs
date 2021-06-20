use actix_web::{client::Client, web::Buf};
use rss::Channel;

pub async fn get_source_info(url: &String) -> Option<Channel> {
    let client = Client::default();
    let response = client
        .get(url)
        .header("User-Agent", "actix-web/3.0")
        .send()
        .await;
    let buf = response.ok()?.body().await.ok()?;
    let channel = Channel::read_from(buf.bytes()).ok()?;
    Some(channel)
}
