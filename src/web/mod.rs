pub mod dashboard;

use axum::Router;

use crate::app::{Error, State};

pub fn router(state: State) -> Router {
    Router::new().merge(dashboard::router()).with_state(state)
}

pub async fn run(state: State) -> Result<(), Error> {
    let address = format!("{}:{}", state.env.WEB_HOST, state.env.WEB_PORT);

    let listener = tokio::net::TcpListener::bind(&address).await?;

    tracing::info!(%address, "Web server listening");

    axum::serve(listener, router(state)).await?;

    Ok(())
}
