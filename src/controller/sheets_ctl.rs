use axum::{
    extract::State,
    response::IntoResponse, Json,
};
use serde::{Deserialize, Serialize};

use crate::{config::state::AppState, service::sheets_svc, util::response_util::ApiResponse};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppendParam {
    pub id: u32,
    pub name: String,
    pub website: String,
    pub channels: String,
    pub groups: String,
    pub collection_time: String,
    pub tags: String,
}

pub async fn append(State(app_state): State<AppState>, body: Json<AppendParam>) -> impl IntoResponse {
    let res = sheets_svc::append(&app_state, &body.0).await;
    if res.is_err() {
        return ApiResponse::error(&res.err().unwrap().to_string());
    }
    ApiResponse::ok_data(res.unwrap())
}
