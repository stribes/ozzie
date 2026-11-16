use std::env;

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct Env {
    pub DISCORD_TOKEN: String,
    pub WEB_HOST: String,
    pub WEB_PORT: u16,
}

impl Env {
    pub fn load() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        dotenvy::dotenv().ok();

        Ok(Self {
            DISCORD_TOKEN: env::var("DISCORD_TOKEN")?,
            WEB_HOST: env::var("WEB_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            WEB_PORT: env::var("WEB_PORT")
                .unwrap_or_else(|_| "3675".into())
                .parse()?,
        })
    }
}
