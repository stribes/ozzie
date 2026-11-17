use serenity::all::Ready;

pub async fn handler(ready: &Ready) {
    tracing::info!("logged in as {} ({})", ready.user.tag(), ready.user.id);
    tracing::info!("connected to {} guilds", ready.guilds.len());
}
