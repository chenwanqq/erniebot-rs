use json_value_merge::Merge;
use serde_json::Value;
use url::Url;

use super::model::Text2ImageModel;
use super::option::Text2ImageOpt;
use super::response::Text2ImageResponse;
use crate::errors::ErnieError;
use crate::utils::{build_url, get_access_token};

static TEXT2IMAGE_BASE_URL: &str =
    "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/text2image/";
/// Text2ImageEndpoint is a struct that represents the text2image endpoint of erniebot API
#[derive(Debug, Clone)]
pub struct Text2ImageEndpoint {
    url: Url,
    access_token: String,
}

impl Text2ImageEndpoint {
    /// create a new text2image instance using pre-defined model
    pub fn new(model: Text2ImageModel) -> Result<Self, ErnieError> {
        Ok(Text2ImageEndpoint {
            url: build_url(TEXT2IMAGE_BASE_URL, model.to_string().as_str())?,
            access_token: get_access_token()?,
        })
    }

    /// create a new text2image instance using custom endpoint
    pub fn new_with_custom_endpoint(endpoint: &str) -> Result<Self, ErnieError> {
        Ok(Text2ImageEndpoint {
            url: build_url(TEXT2IMAGE_BASE_URL, endpoint)?,
            access_token: get_access_token()?,
        })
    }

    fn generate_body(prompt: &str, options: &Vec<Text2ImageOpt>) -> serde_json::Value {
        let mut body = serde_json::json!({
            "prompt": prompt,
        });
        for opt in options {
            body.merge(&serde_json::json!(opt));
        }
        body
    }

    /// sync invoke
    pub fn invoke(
        &self,
        prompt: &str,
        options: &Vec<Text2ImageOpt>,
    ) -> Result<Text2ImageResponse, ErnieError> {
        let body = Text2ImageEndpoint::generate_body(prompt, options);
        let client = reqwest::blocking::Client::new();
        let response: Value = client
            .post(self.url.as_str())
            .query(&[("access_token", self.access_token.as_str())])
            .json(&body)
            .send()
            .map_err(|e| ErnieError::InvokeError(e.to_string()))?
            .json()
            .map_err(|e| ErnieError::InvokeError(e.to_string()))?;

        //if error_code key in response, means RemoteAPIError
        if response.get("error_code").is_some() {
            return Err(ErnieError::RemoteAPIError(response.to_string()));
        }

        Ok(Text2ImageResponse::new(response))
    }

    ///async invoke
    pub async fn ainvoke(
        &self,
        prompt: &str,
        options: &Vec<Text2ImageOpt>,
    ) -> Result<Text2ImageResponse, ErnieError> {
        let body = Text2ImageEndpoint::generate_body(prompt, options);
        let client = reqwest::Client::new();
        let response: Value = client
            .post(self.url.as_str())
            .query(&[("access_token", self.access_token.as_str())])
            .json(&body)
            .send()
            .await
            .map_err(|e| ErnieError::InvokeError(e.to_string()))?
            .json()
            .await
            .map_err(|e| ErnieError::InvokeError(e.to_string()))?;

        //if error_code key in response, means RemoteAPIError
        if response.get("error_code").is_some() {
            return Err(ErnieError::RemoteAPIError(response.to_string()));
        }
        Ok(Text2ImageResponse::new(response))
    }
}
