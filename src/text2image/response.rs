use crate::errors::ErnieError;
use serde::{Deserialize, Serialize};
use serde_json::value;

/// Response is using for non-stream response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Text2ImageResponse {
    raw_response: value::Value,
}

impl Text2ImageResponse {
    pub fn new(raw_response: value::Value) -> Self {
        Text2ImageResponse { raw_response }
    }

    pub fn get_raw_response(&self) -> &value::Value {
        &self.raw_response
    }

    pub fn get(&self, key: &str) -> Option<&value::Value> {
        self.raw_response.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut value::Value> {
        self.raw_response.get_mut(key)
    }

    //return list of image b64 strings
    pub fn get_image_results(&self) -> Result<Vec<String>, ErnieError> {
        match self.raw_response.get("data") {
            Some(data) => {
                let mut result = Vec::new();
                let mut data_array = data
                    .as_array()
                    .ok_or(ErnieError::GetResponseError(
                        "embedding data is not an array".to_string(),
                    ))?
                    .clone();
                data_array.sort_by_key(|a| a.get("index").unwrap().as_i64().unwrap());
                for image_data in data_array {
                    let image = image_data
                        .get("b64_image")
                        .ok_or(ErnieError::GetResponseError(
                            "image is not found".to_string(),
                        ))?;
                    let image_str = image.as_str().ok_or(ErnieError::GetResponseError(
                        "image is not a string".to_string(),
                    ))?;
                    result.push(image_str.to_string());
                }
                Ok(result)
            }
            None => Err(ErnieError::GetResponseError(
                "embedding data is not found".to_string(),
            )),
        }
    }

    pub fn get_prompt_tokens(&self) -> Option<u64> {
        let usage = self.get("usage")?.as_object()?;
        let prompt_tokens = usage.get("prompt_tokens")?.as_u64()?;
        Some(prompt_tokens)
    }

    pub fn get_total_tokens(&self) -> Option<u64> {
        let usage = self.get("usage")?.as_object()?;
        let total_tokens = usage.get("total_tokens")?.as_u64()?;
        Some(total_tokens)
    }
}
