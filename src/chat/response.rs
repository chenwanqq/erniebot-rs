use super::FunctionCall;
use crate::errors::ErnieError;
use serde::{Deserialize, Serialize};
use serde_json::value;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio_stream::Stream;

/// Response is a struct that represents the response of erniebot API.
///
/// It is a wrapper of serde_json::Value. in non-stream case, the API will return a single response, and in stream case, the API will return multiple responses.(see in `Responses` struct)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response {
    raw_response: value::Value,
}

impl Response {
    pub fn new(raw_response: value::Value) -> Self {
        Response { raw_response }
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

    /// get the result of chat response
    pub fn get_chat_result(&self) -> Result<String, ErnieError> {
        match self.raw_response.get("result") {
            Some(result) => match result.as_str() {
                Some(result_str) => Ok(result_str.to_string()),
                None => Err(ErnieError::GetResponseError(
                    "result is not a string".to_string(),
                )),
            },
            None => Err(ErnieError::GetResponseError(
                "result is not found".to_string(),
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
    pub fn get_completion_tokens(&self) -> Option<u64> {
        let usage = self.get("usage")?.as_object()?;
        let completion_tokens = usage.get("completion_tokens")?.as_u64()?;
        Some(completion_tokens)
    }

    /// get total tokens used
    pub fn get_total_tokens(&self) -> Option<u64> {
        let usage = self.get("usage")?.as_object()?;
        let total_tokens = usage.get("total_tokens")?.as_u64()?;
        Some(total_tokens)
    }

    /// get function call
    pub fn get_function_call(&self) -> Option<FunctionCall> {
        let value = self.get("function_call")?;
        let function_call = serde_json::from_value(value.clone()).ok()?;
        Some(function_call)
    }
}

/// Responses is using for sync stream response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Responses {
    responses: Vec<Response>,
}

impl Responses {
    /// get Responses from reqwest::blocking::client. In the response body, it contains multiple responses split by blank line. This method will parse the response body and return a Responses struct.
    pub fn from_text(text: String) -> Result<Self, ErnieError> {
        let parts = text.split("\n\n").collect::<Vec<&str>>();
        let mut result = Vec::new();
        for part in parts {
            if part.is_empty() {
                continue;
            }
            let json_str = part.strip_prefix("data: ");
            match json_str {
                Some(json_str) => {
                    let response: value::Value = serde_json::from_str(json_str)
                        .map_err(|e| ErnieError::GetResponseError(e.to_string()))?;
                    let response = Response::new(response);
                    result.push(response);
                }
                None => {
                    return Err(ErnieError::GetResponseError(format!(
                        "json_str is not found in this part of data: {}",
                        part
                    )))
                }
            }
        }
        Ok(Responses { responses: result })
    }

    /// get chat result as a vector of string
    pub fn get_results(&self) -> Result<Vec<String>, ErnieError> {
        let mut result = Vec::new();
        for response in &self.responses {
            result.push(response.get_chat_result()?);
        }
        Ok(result)
    }

    /// get whole chat result as a single string
    pub fn get_whole_result(&self) -> Result<String, ErnieError> {
        let mut result = String::new();
        for response in &self.responses {
            result.push_str(&response.get_chat_result()?);
        }
        Ok(result)
    }
}

/// StreamResponse is a struct that represents the response of erniebot API in async stream case.
pub struct StreamResponse {
    receiver: UnboundedReceiver<Response>,
}
impl StreamResponse {
    pub fn new() -> (mpsc::UnboundedSender<Response>, Self) {
        let (sender, receiver) = mpsc::unbounded_channel();
        (sender, Self { receiver })
    }
}

impl Stream for StreamResponse {
    type Item = Response;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}
