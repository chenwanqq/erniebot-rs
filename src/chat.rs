use crate::message;

use super::errors::ChatError;
use super::message::{Message, Role};
use super::model::ChatModel;
use super::option::Opt;
use super::response::{Response, Responses, StreamResponse};
use super::utils::get_access_token;
use json_value_merge::Merge;
use reqwest_eventsource::{Event, EventSource, RequestBuilderExt};
use serde_json::Value;
use tokio_stream::StreamExt;
use url::Url;

static CHAT_API_URL: &'static str =
    "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/";
static CUSTOM_API_URL: &'static str =
    "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/";

//TODO: finish this
struct Chat {
    url: Url,
    access_token: String,
}

impl Chat {
    /// Build the url for the chat model
    fn build_url(url: &str, model: &str) -> Result<Url, ChatError> {
        let base = Url::parse(url)?;
        let joined = base.join(model)?;
        Ok(joined)
    }

    /// create a new chat instance using pre-defined model
    pub fn new(model: ChatModel) -> Result<Self, ChatError> {
        Ok(Chat {
            url: Chat::build_url(CHAT_API_URL, model.to_string().as_str())?,
            access_token: get_access_token()?,
        })
    }

    /// create a new chat instance using custom model release on https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/{custom_endpoint}
    pub fn new_with_custom_endpoint(endpoint: &str) -> Result<Self, ChatError> {
        Ok(Chat {
            url: Chat::build_url(CUSTOM_API_URL, endpoint)?,
            access_token: get_access_token()?,
        })
    }

    fn generate_body(
        messages: Vec<Message>,
        options: Vec<Opt>,
        stream: bool,
    ) -> Result<serde_json::Value, ChatError> {
        let mut body = serde_json::json!({
            "messages": messages,
        });
        let options_array = serde_json::json!(options).as_array().cloned();
        if options_array.is_none() {
            return Err(ChatError::GenerateBodyError(
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

    pub fn invoke(&self, messages: Vec<Message>, options: Vec<Opt>) -> Result<Response, ChatError> {
        let body = Chat::generate_body(messages, options, false)?;
        let client = reqwest::blocking::Client::new();
        let response: Value = client
            .post(self.url.as_str())
            .header("Content-Type", "application/json")
            .query(&[("access_token", self.access_token.as_str())])
            .json(&body)
            .send()
            .map_err(|e| ChatError::InvokeError(e.to_string()))?
            .json()
            .map_err(|e| ChatError::InvokeError(e.to_string()))?;

        //if error_code key in response, means RemoteAPIError
        if response.get("error_code").is_some() {
            return Err(ChatError::RemoteAPIError(response.to_string()));
        }
        Ok(Response::new(response))
    }
    pub fn stream(
        &self,
        messages: Vec<Message>,
        options: Vec<Opt>,
    ) -> Result<Responses, ChatError> {
        let body = Chat::generate_body(messages, options, true)?;
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(self.url.as_str())
            .header("Content-Type", "application/json")
            .query(&[("access_token", self.access_token.as_str())])
            .json(&body)
            .send()
            .map_err(|e| ChatError::StreamError(e.to_string()))?
            .text()
            .map_err(|e| ChatError::StreamError(e.to_string()))?;
        let response = Responses::from_text(response)?;
        Ok(response)
    }

    pub async fn ainvoke(
        &self,
        messages: Vec<Message>,
        options: Vec<Opt>,
    ) -> Result<Response, ChatError> {
        let body = Chat::generate_body(messages, options, false)?;
        let client = reqwest::Client::new();
        let response: Value = client
            .post(self.url.as_str())
            .header("Content-Type", "application/json")
            .query(&[("access_token", self.access_token.as_str())])
            .json(&body)
            .send()
            .await
            .map_err(|e| ChatError::InvokeError(e.to_string()))?
            .json()
            .await
            .map_err(|e| ChatError::InvokeError(e.to_string()))?;

        //if error_code key in response, means RemoteAPIError
        if response.get("error_code").is_some() {
            return Err(ChatError::RemoteAPIError(response.to_string()));
        }
        Ok(Response::new(response))
    }

    pub async fn astream(
        &self,
        messages: Vec<Message>,
        options: Vec<Opt>,
    ) -> Result<StreamResponse, ChatError> {
        let body = Chat::generate_body(messages, options, true)?;
        let client = reqwest::Client::new();
        let mut event_source = client
            .post(self.url.as_str())
            .header("Content-Type", "application/json")
            .query(&[("access_token", self.access_token.as_str())])
            .json(&body)
            .eventsource()
            .map_err(|e| ChatError::StreamError(e.to_string()))?;
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
    use super::{Chat, Message, Opt, Role};
    use tokio::runtime::Runtime;
    use tokio_stream::StreamExt;

    #[test]
    fn test_generate_body() {
        let messages = vec![Message {
            role: Role::User,
            content: "hello, I'm a user".to_string(),
            name: None,
        }];
        let options = vec![Opt::Temperature(0.5), Opt::TopP(0.5), Opt::TopK(50)];
        let result = Chat::generate_body(messages, options, true).unwrap();
        let s = serde_json::to_string(&result).unwrap();
        println!("{}", s);
    }

    #[test]
    fn test_invoke() {
        let chat = Chat::new(crate::model::ChatModel::ErnieBotTurbo).unwrap();
        let messages = vec![
            Message {
                role: Role::User,
                content: "hello, I'm a developer. I'm developing a rust SDK for qianfan LLM. If you get this message, that means I successfully send you this message using invoke method".to_string(),
                name: None,
            },
        ];
        let options = vec![Opt::Temperature(0.5), Opt::TopP(0.5), Opt::TopK(50)];
        let response = chat.invoke(messages, options).unwrap();
        let result = response.get_result().unwrap();
        println!("{}", result);
    }

    #[test]
    fn test_stream() {
        let chat = Chat::new(crate::model::ChatModel::ErnieBotTurbo).unwrap();
        let messages = vec![
            Message {
                role: Role::User,
                content: "hello, I'm a developer. I'm developing a rust SDK for qianfan LLM. If you get this message, that means I successfully send you this message using stream method".to_string(),
                name: None,
            },
        ];
        let options = vec![Opt::Temperature(0.5), Opt::TopP(0.5), Opt::TopK(50)];
        let response = chat.stream(messages, options).unwrap();
        let result_by_chunk = response.get_results().unwrap();
        println!("{:?}", result_by_chunk);
        let whole_result = response.get_whole_result().unwrap();
        println!("{}", whole_result);
    }

    #[test]
    fn test_ainvoke() {
        let chat = Chat::new(crate::model::ChatModel::ErnieBotTurbo).unwrap();
        let messages = vec![
            Message {
                role: Role::User,
                content: "hello, I'm a developer. I'm developing a rust SDK for qianfan LLM. If you get this message, that means I successfully send you this message using ainvoke method".to_string(),
                name: None,
            },
        ];
        let options = Vec::new();
        let rt = Runtime::new().unwrap();
        let response = rt.block_on(chat.ainvoke(messages, options)).unwrap();
        let result = response.get_result().unwrap();
        println!("{}", result);
    }

    #[test]
    fn test_astream() {
        let chat = Chat::new(crate::model::ChatModel::ErnieBotTurbo).unwrap();
        let messages = vec![
            Message {
                role: Role::User,
                content: "hello, I'm a developer. I'm developing a rust SDK for qianfan LLM. If you get this message, that means I successfully send you this message using async stream method. Now reply to me a message as long as possible so that I can test if this function doing well".to_string(),
                name: None,
            },
        ];
        let options = Vec::new();
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let mut stream_response = chat.astream(messages, options).await.unwrap();
            while let Some(response) = stream_response.next().await {
                let result = response.get_result().unwrap();
                println!("{}", result);
            }
        })
    }
}
