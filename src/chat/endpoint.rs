use super::message::Message;
use super::model::ChatModel;
use super::option::ChatOpt;
use super::response::{Response, Responses, StreamResponse};

use crate::errors::ErnieError;
use crate::utils::{build_url, get_access_token};
use json_value_merge::Merge;
use reqwest_eventsource::{Event, RequestBuilderExt};
use serde_json::Value;
use tokio_stream::StreamExt;
use url::Url;

static CHAT_API_URL: &str = "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/";

/** ChatEndpoint is a struct that represents the chat endpoint of erniebot API
*/
#[derive(Debug, Clone)]
pub struct ChatEndpoint {
    url: Url,
    access_token: String,
}

impl ChatEndpoint {
    /// create a new chat instance using pre-defined model
    pub fn new(model: ChatModel) -> Result<Self, ErnieError> {
        Ok(ChatEndpoint {
            url: build_url(CHAT_API_URL, model.to_string().as_str())?,
            access_token: get_access_token()?,
        })
    }

    /// create a new chat instance using custom model release on https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/{custom_endpoint}
    pub fn new_with_custom_endpoint(endpoint: &str) -> Result<Self, ErnieError> {
        Ok(ChatEndpoint {
            url: build_url(CHAT_API_URL, endpoint)?,
            access_token: get_access_token()?,
        })
    }

    fn generate_body(
        messages: &Vec<Message>,
        options: &Vec<ChatOpt>,
        stream: bool,
    ) -> Result<serde_json::Value, ErnieError> {
        let mut body = serde_json::json!({
            "messages": messages,
        });
        let options_array = serde_json::json!(options).as_array().cloned();
        if options_array.is_none() {
            return Err(ErnieError::GenerateBodyError(
                "options is not valid".to_string(),
            ));
        }
        let options_array = options_array.unwrap();
        for opt in options_array {
            body.merge(&opt);
        }
        if stream {
            body.merge(&serde_json::json!({"stream": true}));
        }
        Ok(body)
    }

    /// invoke method is used to send a request to erniebot chat endpoint. This is a blocking method that will return a full response from the chat endpoint
    pub fn invoke(
        &self,
        messages: &Vec<Message>,
        options: &Vec<ChatOpt>,
    ) -> Result<Response, ErnieError> {
        let body = ChatEndpoint::generate_body(messages, options, false)?;
        let response: Value = ureq::post(self.url.as_str())
            .set("Content-Type", "application/json")
            .query("access_token", self.access_token.as_str())
            .send_json(body)
            .map_err(|e| ErnieError::InvokeError(e.to_string()))?
            .into_json()
            .map_err(|e| ErnieError::InvokeError(e.to_string()))?;

        //if error_code key in response, means RemoteAPIError
        if response.get("error_code").is_some() {
            return Err(ErnieError::RemoteAPIError(response.to_string()));
        }
        Ok(Response::new(response))
    }
    /// stream method is used to send a request to erniebot chat endpoint. This is a blocking method that will return response in multiple chunks from the chat endpoint
    pub fn stream(
        &self,
        messages: &Vec<Message>,
        options: &Vec<ChatOpt>,
    ) -> Result<Responses, ErnieError> {
        let body = ChatEndpoint::generate_body(messages, options, true)?;
        let response: String = ureq::post(self.url.as_str())
            .set("Content-Type", "application/json")
            .query("access_token", self.access_token.as_str())
            .send_json(body)
            .map_err(|e| ErnieError::InvokeError(e.to_string()))?
            .into_string()
            .map_err(|e| ErnieError::InvokeError(e.to_string()))?;
        let response = Responses::from_text(&response)?;
        Ok(response)
    }

    /// ainvoke method is used to send a request to erniebot chat endpoint. This is an async method that will return a full response from the chat endpoint
    pub async fn ainvoke(
        &self,
        messages: &Vec<Message>,
        options: &Vec<ChatOpt>,
    ) -> Result<Response, ErnieError> {
        let body = ChatEndpoint::generate_body(messages, options, false)?;
        let client = reqwest::Client::new();
        let response: Value = client
            .post(self.url.as_str())
            .header("Content-Type", "application/json")
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
        Ok(Response::new(response))
    }

    /// astream method is used to send a request to erniebot chat endpoint. This is an async method that will return response in multiple chunks from the chat endpoint
    pub async fn astream(
        &self,
        messages: &Vec<Message>,
        options: &Vec<ChatOpt>,
    ) -> Result<StreamResponse, ErnieError> {
        let body = ChatEndpoint::generate_body(messages, options, true)?;
        let client = reqwest::Client::new();
        let mut event_source = client
            .post(self.url.as_str())
            .header("Content-Type", "application/json")
            .query(&[("access_token", self.access_token.as_str())])
            .json(&body)
            .eventsource()
            .map_err(|e| ErnieError::StreamError(e.to_string()))?;
        let (sender, stream_response) = StreamResponse::new();
        tokio::spawn(async move {
            while let Some(event) = event_source.next().await {
                if event.is_err() {
                    break;
                }
                let event = event.unwrap();
                match event {
                    Event::Open => continue,
                    Event::Message(message_event) => {
                        let data = &message_event.data;
                        match serde_json::from_str(data) {
                            Ok(value) => {
                                let response = Response::new(value);
                                sender.send(response).unwrap();
                            }
                            Err(_) => {
                                break;
                            }
                        }
                    }
                }
            }
        });
        Ok(stream_response)
    }
}

#[cfg(test)]
mod tests {
    use crate::chat::{ChatEndpoint, ChatOpt, Message, Role};
    #[test]
    fn test_generate_body() {
        let messages = vec![Message {
            role: Role::User,
            content: "hello, I'm a user".to_string(),
            ..Default::default()
        }];
        let options = vec![
            ChatOpt::Temperature(0.5),
            ChatOpt::TopP(0.5),
            ChatOpt::TopK(50),
        ];
        let result = ChatEndpoint::generate_body(&messages, &options, true).unwrap();
        let s = serde_json::to_string(&result).unwrap();
        println!("{}", s);
    }
}
