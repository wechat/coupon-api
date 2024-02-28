use axum::http::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub const CODE_SUCCESS: StatusCode = StatusCode::OK;
pub const CODE_FAIL: StatusCode = StatusCode::BAD_REQUEST;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseObject<T> {
    pub code: Option<u16>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> ResponseObject<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &T) -> Self {
        Self {
            code: Some(CODE_SUCCESS.as_u16()),
            msg: Some("success".to_string()),
            data: Some(arg.clone()),
        }
    }

    pub fn from_result_tip(arg: &str) -> Self {
        Self {
            code: Some(CODE_SUCCESS.as_u16()),
            msg: Some(arg.to_string()),
            data: None,
        }
    }

    pub fn from_error(arg: &str) -> Self {
        Self {
            code: Some(CODE_FAIL.as_u16()),
            msg: Some(arg.to_string()),
            data: None,
        }
    }

    pub fn from_error_info(code: StatusCode, info: &str) -> Self {
        Self {
            code: Some(code.as_u16()),
            msg: Some(info.to_string()),
            data: None,
        }
    }

    pub fn from_error_infos(info: &str) -> Self {
        Self {
            code: Some(CODE_FAIL.as_u16()),
            msg: Some(info.to_string()),
            data: None,
        }
    }
}
