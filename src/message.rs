use serde::{Deserialize, Serialize};
use serde_json::to_string;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Message {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Message;
    use serde_json::to_string;
    #[test]
    fn test_message() {
        let message1 = Message {
            role: "robot".to_string(),
            content: "hello, I'm a AI LLM model".to_string(),
            name: None,
        };
        let message2 = Message {
            role: "user".to_string(),
            content: "hello, I'm a user".to_string(),
            name: None,
        };
        let message3 = Message {
            role: "robot".to_string(),
            content: "hello, I'm a AI LLM model".to_string(),
            name: None,
        };
        let messages = vec![message1, message2, message3];
        let messages_str = to_string(&messages).unwrap();
        println!("{}", messages_str);
    }
}




