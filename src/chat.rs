use super::model::ChatModel;
use super::utils::get_access_token;
use url::{ParseError, Url};
use super::message::Message;
use super::option::Opt;

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
    fn build_url(url: &str, model: &str) -> Result<Url, ParseError> {
        let base = Url::parse(url).expect("url is invalid");
        let joined = base.join(model)?;
        Ok(joined)
    }

    /// create a new chat instance using pre-defined model
    fn new(model: ChatModel) -> Self {
        Chat {
            url: Chat::build_url(CHAT_API_URL, model.to_string().as_str()).unwrap(),
            access_token: get_access_token().unwrap(),
        }
    }

    /// create a new chat instance using custom model release on https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/{custom_endpoint}
    fn new_with_custom_endpoint(endpoint: &str) -> Self {
        Chat {
            url: Chat::build_url(CUSTOM_API_URL, endpoint).unwrap(),
            access_token: get_access_token().unwrap(),
        }
    }

    fn invoke(&self, messages: Vec<Message>, options: Vec<Opt>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
