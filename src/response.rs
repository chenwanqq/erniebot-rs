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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamResponse {
    responses: Vec<Response>,
}

impl StreamResponse {
    pub fn from_text(text: String) -> Result<Self,GetResponseError> {
        let parts = text.split("\n\n").collect::<Vec<&str>>();
        let mut result = Vec::new();
        for part in parts {
            if part.is_empty() {
                continue;
            }
            let json_str = part.strip_prefix("data: ");
            match json_str {
                Some(json_str) => {
                    let response: value::Value = serde_json::from_str(json_str).map_err(|e| GetResponseError::Details(e.to_string()))?;
                    let response = Response::new(response);
                    result.push(response);
                },
                None => {
                    return Err(GetResponseError::Details(format!("json_str is not found in this part of data: {}", part)))
                }
            }
        }
        Ok(StreamResponse {
            responses: result
        })
    }
    pub fn get_results(&self) -> Result<Vec<String>,GetResponseError> {
        let mut result = Vec::new();
        for response in &self.responses {
            result.push(response.get_result()?);
        }
        Ok(result)
    }

    pub fn get_whole_result(&self) -> Result<String,GetResponseError> {
        let mut result = String::new();
        for response in &self.responses {
            result.push_str(&response.get_result()?);
        }
        Ok(result)
    }
}