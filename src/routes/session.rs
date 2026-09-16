use std::sync::Arc;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use cookie::Cookie;

pub async fn handle(state: axum::extract::State<Arc<crate::state::AppStateManager>>) -> Response {
    let session = state.session.lock().unwrap();
    let token = session.token.clone();

    let cookie = cookie::Cookie::build(("session_id", token.clone()))
        .path("/")
        .http_only(false)
        .same_site(cookie::SameSite::Lax)
        .max_age(cookie::time::Duration::hours(24))
        .build();

    let mut response = (StatusCode::OK, "{\"active\": true}").into_response();
    response.headers_mut().insert(
        axum::http::header::SET_COOKIE,
        axum::http::HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );
    response
}
