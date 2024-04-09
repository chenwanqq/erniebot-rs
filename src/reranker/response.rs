use crate::errors::ErnieError;
use serde::{Deserialize, Serialize};
use serde_json::value;

/// Response is using for non-stream response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RerankerResponse {
    raw_response: value::Value,
}

impl RerankerResponse {
    pub fn new(raw_response: value::Value) -> Self {
        RerankerResponse { raw_response }
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

    /// get the result of reranker response
    pub fn get_reranker_response(&self) -> Result<Vec<RerankData>, ErnieError> {
        match self.raw_response.get("results") {
            Some(data) => {
                let data_array = data
                    .as_array()
                    .ok_or(ErnieError::GetResponseError(
                        "reranker results is not an array".to_string(),
                    ))?
                    .clone();
                let results = data_array
                    .into_iter()
                    .map(|x| {
                        serde_json::from_value(x)
                            .map_err(|e| ErnieError::GetResponseError(e.to_string()))
                    })
                    .collect::<Result<Vec<RerankData>, ErnieError>>()?;
                Ok(results)
            }
            None => Err(ErnieError::GetResponseError(
                "reranker results is not found".to_string(),
            )),
        }
    }
    /// get tokens used by prompt
    pub fn get_prompt_tokens(&self) -> Option<u64> {
        let usage = self.get("usage")?.as_object()?;
        let prompt_tokens = usage.get("prompt_tokens")?.as_u64()?;
        Some(prompt_tokens)
    }
    /// get tokens used by completion
    pub fn get_total_tokens(&self) -> Option<u64> {
        let usage = self.get("usage")?.as_object()?;
        let total_tokens = usage.get("total_tokens")?.as_u64()?;
        Some(total_tokens)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RerankData {
    pub document: String,
    pub relevance_score: f64,
    pub index: u64,
}
