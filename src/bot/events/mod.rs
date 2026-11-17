pub mod ready;

use serenity::all::FullEvent;

pub async fn handle(event: &FullEvent) {
    if let FullEvent::Ready { data_about_bot, .. } = event {
        ready::handler(data_about_bot).await;
    }
}
