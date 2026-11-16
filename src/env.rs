use std::env;

#[allow(non_snake_case)]
pub struct Env {
    pub WEB_HOST: String,
    pub WEB_PORT: u16,
}

impl Env {
    pub fn load() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        dotenvy::dotenv().ok();

        Ok(Self {
            WEB_HOST: env::var("WEB_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            WEB_PORT: env::var("WEB_PORT")
                .unwrap_or_else(|_| "3675".into())
                .parse()?,
        })
    }
}
