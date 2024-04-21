use super::model::EmbeddingModel;
use super::response::EmbeddingResponse;
use crate::errors::ErnieError;
use crate::utils::{build_url, get_access_token};
use json_value_merge::Merge;
use serde_json::Value;
use url::Url;

static EMBEDDING_BASE_URL: &str =
    "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/embeddings/";

/** ChatEndpoint is a struct that represents the chat endpoint of erniebot API
*/
#[derive(Debug, Clone)]
pub struct EmbeddingEndpoint {
    url: Url,
    access_token: String,
}

impl EmbeddingEndpoint {
    // create a new embedding instance using pre-defined model
    pub fn new(model: EmbeddingModel) -> Result<Self, ErnieError> {
        Ok(EmbeddingEndpoint {
            url: build_url(EMBEDDING_BASE_URL, model.to_string().as_str())?,
            access_token: get_access_token()?,
        })
    }
    /// sync invoke
    pub fn invoke(
        &self,
        input: &Vec<String>,
        user_id: Option<&str>,
    ) -> Result<EmbeddingResponse, ErnieError> {
        let mut body = serde_json::json!({
            "input": input,
        });
        if let Some(user_id) = user_id {
            body.merge(&serde_json::json!({"user_id": user_id}));
        }
        let response: Value = ureq::post(self.url.as_str())
            .query("access_token", self.access_token.as_str())
            .send_json(body)
            .map_err(|e| ErnieError::InvokeError(e.to_string()))?
            .into_json()
            .map_err(|e| ErnieError::InvokeError(e.to_string()))?;

        //if error_code key in response, means RemoteAPIError
        if response.get("error_code").is_some() {
            return Err(ErnieError::RemoteAPIError(response.to_string()));
        }

        Ok(EmbeddingResponse::new(response))
    }
    ///async invoke
    pub async fn ainvoke(
        &self,
        input: &Vec<String>,
        user_id: Option<&str>,
    ) -> Result<EmbeddingResponse, ErnieError> {
        let mut body = serde_json::json!({
            "input": input,
        });
        if let Some(user_id) = user_id {
            body.merge(&serde_json::json!({"user_id": user_id}));
        }
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

        Ok(EmbeddingResponse::new(response))
    }
}
