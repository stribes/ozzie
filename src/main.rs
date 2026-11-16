use tracing_subscriber::EnvFilter;

pub mod app;
pub mod bot;
pub mod env;
pub mod web;

#[tokio::main]
async fn main() -> Result<(), app::Error> {
    init_tracing();

    let env = env::Env::load()?;

    app::run(env).await
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
}
