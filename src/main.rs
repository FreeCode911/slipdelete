use std::sync::Arc;
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};

mod error;
mod media;
mod recycle_bin;
mod routes;
mod state;

use crate::routes::session;
use state::AppStateManager;

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppStateManager::new());

    let app = Router::new()
        .route("/", get(index))
        .route("/api/session", get(session::handle))
        .route("/api/state", get(routes::state::handle))
        .route("/api/media", get(routes::media::handle))
        .route("/api/delete", post(routes::actions::delete))
        .route("/api/keep", post(routes::actions::keep))
        .route("/api/restore", post(routes::actions::restore))
        .route("/api/reset", post(routes::actions::reset))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("Server running on http://0.0.0.0:5000");
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html(include_str!("template.html"))
}
