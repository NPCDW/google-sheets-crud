use crate::config::state::AppState;

use anyhow::anyhow;
use google_cloud_auth::{credentials::CredentialsFile, project::{create_token_source_from_credentials, Config}};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

const USER_AGENT_KEY: &str = "User-Agent";
const USER_AGENT_VALUE: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VpsDetail {
    pub id: u32,
    pub name: String,
    pub website: String,
    pub channels: String,
    pub groups: String,
    pub collection_time: String,
    pub tags: String,
}

pub async fn update(app_state: &AppState, body: &VpsDetail) -> anyhow::Result<()> {
    let token = get_token(app_state).await?;
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT_KEY, HeaderValue::from_static(USER_AGENT_VALUE));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let range = format!("A{}", body.id + 1);
    let url = reqwest::Url::parse(&format!("https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?valueInputOption=RAW&access_token={}", app_state.config.sheets_id, &range, token))?;
    let mut map = serde_json::Map::new();
    map.insert("range".to_string(), serde_json::Value::String(range));
    map.insert("values".to_string(), serde_json::Value::Array(vec![serde_json::Value::Array(vec![
        body.id.into(),
        body.name.clone().into(),
        body.website.clone().into(),
        body.channels.clone().into(),
        body.groups.clone().into(),
        body.collection_time.clone().into(),
        body.tags.clone().into()])]));
    let response = client.put(url).body(serde_json::to_string(&map)?).send().await?;
    if !response.status().is_success() {
        return Err(anyhow!("Error: {:?} Error Body: {:?}", response.status(), response.text().await));
    }
    anyhow::Ok(())
}

pub async fn get_token(app_state: &AppState) -> anyhow::Result<String> {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs();
    let token_exp = { app_state.token_exp.read().await.clone() };
    if token_exp.is_some() && token_exp.unwrap() > timestamp {
        return anyhow::Ok(app_state.token.read().await.clone().unwrap());
    }
    tracing::info!("token 过期正在重新获取 token");
    let scopes = [
        "https://www.googleapis.com/auth/drive",
        "https://www.googleapis.com/auth/drive.file",
        "https://www.googleapis.com/auth/spreadsheets"
    ];
    let config = Config {
        audience: None,
        scopes: Some(&scopes),
        sub: None
    };
    let credentials_file = CredentialsFile::new_from_file("service_account.json".to_string()).await?;
    let ts = create_token_source_from_credentials(&credentials_file, &config).await?;
    let token = ts.token().await?;
    tracing::debug!("token is {}", token.access_token);
    {
        *app_state.token.write().await = Some(token.access_token.clone());
        *app_state.token_exp.write().await = Some(timestamp + (40 * 60));
    }
    anyhow::Ok(token.access_token)
}