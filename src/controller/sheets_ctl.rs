use axum::{
    extract::State,
    response::IntoResponse, Json,
};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{config::state::AppState, service::sheets_svc::{self, VpsDetail}, util::response_util::ApiResponse};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppendParam {
    pub id: i64,
    pub content: String,
}

pub async fn update(State(app_state): State<AppState>, body: Json<AppendParam>) -> impl IntoResponse {
    if !(body.content.contains("➡️序号") && body.content.contains("📋名称") && body.content.contains("🌐官网")) {
        return ApiResponse::ok_msg("该消息非vps商家信息");
    }
    let res = get_vps_detail(&body.content);
    if res.is_err() {
        return ApiResponse::error(&res.err().unwrap().to_string());
    }
    let res = sheets_svc::update(&app_state, &res.unwrap()).await;
    if res.is_err() {
        return ApiResponse::error(&res.err().unwrap().to_string());
    }
    // google sheets api 限制每个用户每分钟操作60次，为了简单，这里直接睡眠1s
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    ApiResponse::ok_data(res.unwrap())
}

fn get_vps_detail(content: &str) -> anyhow::Result<VpsDetail> {
    let number_reg = Regex::new("(?m)^➡️序号:(.*)")?;
    let name_reg = Regex::new("(?m)^📋名称:(.*)")?;
    let website_reg = Regex::new("(?m)^🌐官网:(.*)")?;
    let channels_reg = Regex::new("(?m)^📢频道:(.*)")?;
    let groups_reg = Regex::new("(?m)^💬群组:(.*)")?;
    let collection_time_reg = Regex::new("(?m)^🗓时间:(.*)")?;
    let tags_reg = Regex::new("(?m)^(🏷️标签:|🏷标签:)(.*)")?;
    let number = match number_reg.captures(&content) {
        Some(capture) => capture[1].trim().to_string(),
        None => "".to_string(),
    };
    let name = match name_reg.captures(content) {
        Some(capture) => capture[1].trim().to_string(),
        None => "".to_string(),
    };
    let website = match website_reg.captures(content) {
        Some(capture) => capture[1].trim().to_string(),
        None => "".to_string(),
    };
    let channels = match channels_reg.captures(content) {
        Some(capture) => capture[1].trim().to_string(),
        None => "".to_string(),
    };
    let groups = match groups_reg.captures(content) {
        Some(capture) => capture[1].trim().to_string(),
        None => "".to_string(),
    };
    let collection_time = match collection_time_reg.captures(content) {
        Some(capture) => capture[1].trim().to_string(),
        None => "".to_string(),
    };
    let tags = match tags_reg.captures(content) {
        Some(capture) => capture[2].trim().to_string(),
        None => "".to_string(),
    };
    anyhow::Ok(VpsDetail {
        id: number.parse::<u32>()?,
        name,
        website,
        channels,
        groups,
        collection_time,
        tags,
    })
}