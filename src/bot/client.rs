use poise::Framework;
use serenity::{Client, all::GatewayIntents};

use crate::app::State;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn run(state: State) -> Result<(), Error> {
    let intents = GatewayIntents::non_privileged();

    let token = state.env.DISCORD_TOKEN.clone();

    let framework = Framework::<State, Error>::builder()
        .options(poise::FrameworkOptions::default())
        .setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(state) }))
        .build();

    let mut client = Client::builder(token, intents) //
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}
