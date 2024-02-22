use serde_json::value;
use serde::{Deserialize, Serialize};
use super::errors::GetResponseError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response {
    raw_response: value::Value,
}

impl Response {
    pub fn new(raw_response: value::Value) -> Self {
        Response {
            raw_response
        }
    }

    pub fn get_raw_response(&self) -> &value::Value {
        &self.raw_response
    }

    pub fn get_result(&self) -> Result<String,GetResponseError> {
        match self.raw_response.get("result") {
            Some(result) => {
                match result.as_str() {
                    Some(result_str) => Ok(result_str.to_string()),
                    None => Err(GetResponseError::Details("result is not a string".to_string()))
                }
            },
            None => Err(GetResponseError::Details("result is not found".to_string()))
        }
    }
}