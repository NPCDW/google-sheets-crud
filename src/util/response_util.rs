use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl<T: Serialize> ApiResponse<T> {
    #[allow(dead_code)]
    pub fn ok() -> Self {
        ApiResponse {
            code: 200,
            message: "success".to_string(),
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn ok_msg(message: &str) -> Self {
        ApiResponse {
            code: 200,
            message: message.to_string(),
            data: None,
        }
    }
    
    #[allow(dead_code)]
    pub fn ok_data(data: T) -> Self {
        ApiResponse {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }
    
    #[allow(dead_code)]
    pub fn error(message: &str) -> Self {
        ApiResponse {
            code: 500,
            message: message.to_string(),
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct PageInfo<T> {
    count: u32,
    list: Vec<T>
}

impl<T: Serialize> PageInfo<T> {
    #[allow(dead_code)]
    pub fn new(count: u32, list: Vec<T>) -> Self {
        PageInfo {
            count,
            list,
        }
    }
}
