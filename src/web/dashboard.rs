use std::path::PathBuf;

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use super::State;

pub fn router() -> Router<State> {
    let dashboard_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dashboard");

    let service =
        ServeDir::new(&dashboard_dir).fallback(ServeFile::new(dashboard_dir.join("index.html")));

    Router::new().fallback_service(service)
}
