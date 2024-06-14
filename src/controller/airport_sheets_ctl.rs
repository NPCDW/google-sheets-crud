use axum::{
    extract::State,
    response::IntoResponse, Json,
};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{config::state::AppState, service::sheets_svc, util::response_util::ApiResponse};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateParam {
    pub id: i64,
    pub content: String,
}

pub async fn update(State(app_state): State<AppState>, body: Json<UpdateParam>) -> impl IntoResponse {
    if !(body.content.contains("⦁ 序号") && body.content.contains("⦁ 名称") && body.content.contains("⦁ 官网")) {
        return ApiResponse::ok_msg("该消息非机场商家信息");
    }
    let res = get_airport_detail(&body.content);
    if res.is_err() {
        return ApiResponse::error(&res.err().unwrap().to_string());
    }
    let res = res.unwrap();
    let data = vec![serde_json::Value::Array(vec![
        res.id.into(),
        res.name.clone().into(),
        res.website.clone().into(),
        res.channels.clone().into(),
        res.groups.clone().into(),
        res.collection_time.clone().into()])];
    let res = sheets_svc::update(&app_state, &app_state.config.airport_sheets_id, res.id + 1, data).await;
    if res.is_err() {
        return ApiResponse::error(&res.err().unwrap().to_string());
    }
    // google sheets api 限制每个用户每分钟操作60次，为了简单，这里直接睡眠1s
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    ApiResponse::ok_data(res.unwrap())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AirportDetail {
    pub id: u32,
    pub name: String,
    pub website: String,
    pub channels: String,
    pub groups: String,
    pub collection_time: String,
}

fn get_airport_detail(content: &str) -> anyhow::Result<AirportDetail> {
    let number_reg = Regex::new("(?m)^⦁ 序号:(.*)")?;
    let name_reg = Regex::new("(?m)^⦁ 名称:(.*)")?;
    let website_reg = Regex::new(r"(?m)^⦁ 官网:((\s*http[s]?://.*)|(\s*\*\s*http[s]?://.*)*)")?;
    let channels_reg = Regex::new("(?m)^⦁ 频道:(.*)")?;
    let groups_reg = Regex::new("(?m)^⦁ 群组:(.*)")?;
    let collection_time_reg = Regex::new("(?m)^⦁ 时间:(.*)")?;
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
    anyhow::Ok(AirportDetail {
        id: number.parse::<u32>()?,
        name,
        website,
        channels,
        groups,
        collection_time,
    })
}