use std::any::Any;

use axum::http::Response;
use axum::response::IntoResponse;
use axum::{
    routing::post,
    Router,
};
use tower_http::catch_panic::CatchPanicLayer;
use crate::controller::sheets_ctl;
use crate::config::state::AppState;
use crate::util::response_util::ApiResponse;

pub async fn init(app_state: AppState) -> Router {
    let sheets = Router::new()
        .route("/append", post(sheets_ctl::append));

    let api = Router::new()
        .nest("/sheets", sheets);

    let router = Router::new()
        .nest("/api", api);
    router
        .layer(CatchPanicLayer::custom(handle_panic))
        .with_state(app_state)
}

fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response<axum::body::Body> {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };

    ApiResponse::<()>::error(&details).into_response()
}