use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use serenity::{Client, all::GatewayIntents};

use crate::{
    app::{Error, State},
    bot::events,
};

pub async fn run(state: State) -> Result<(), Error> {
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let token = state.env.DISCORD_TOKEN.clone();

    let framework = Framework::<State, Error>::builder()
        .options(FrameworkOptions {
            //commands: vec![test::test()],
            prefix_options: PrefixFrameworkOptions {
                dynamic_prefix: Some(|ctx| {
                    Box::pin(async move {
                        let state = ctx.framework.user_data;

                        if let Some(guild_id) = ctx.guild_id
                            && let Some(guild) =
                                state.mongo.guilds.get(guild_id.get() as i64).await?
                            && !guild.prefix.is_empty()
                        {
                            return Ok(Some(guild.prefix.into()));
                        }

                        let prefix = state.mongo.config.get().await?.prefix;

                        Ok(Some(prefix.into()))
                    })
                }),
                ..Default::default()
            },
            event_handler: |_framework, event| {
                Box::pin(async move {
                    events::handle(event).await;
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(state) }))
        .build();

    let mut client = Client::builder(token, intents) //
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}
