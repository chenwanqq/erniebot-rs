use super::errors::ChatError;
use super::message::{Message,Role};
use super::model::ChatModel;
use super::option::Opt;
use super::response::Response;
use json_value_merge::Merge;
use serde_json::Value;
use super::utils::get_access_token;
use url::{ParseError, Url};

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
        Ok(body)
    }
    
    pub fn invoke(
        &self,
        messages: Vec<Message>,
        options: Vec<Opt>,
    ) -> Result<Response, ChatError> {
        let body = Chat::generate_body(messages, options)?;
        let client = reqwest::blocking::Client::new();
        let response:Value = client
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
    
}

#[cfg(test)]
mod tests {
    use super::{Chat, Message, Opt,Role};
    #[test]
    fn test_generate_body() {
        let messages = vec![
            Message {
                role: Role::User,
                content: "hello, I'm a user".to_string(),
                name: None,
            },
        ];
        let options = vec![Opt::Temperature(0.5), Opt::TopP(0.5), Opt::TopK(50)];
        let result = Chat::generate_body(messages, options).unwrap();
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
}
