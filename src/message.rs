use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub enum Role {
    User,
    Assistant,
    Function,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub role: Role,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{Message,Role};
    use serde_json::{to_string, self};
    #[test]
    fn test_message() {
        let message1 = Message {
            role: Role::User,
            content: "hello, I'm a user".to_string(),
            name: None,
        };
        let message2 = Message {
            role: Role::Assistant,
            content: "hello, I'm a AI LLM model".to_string(),
            name: None,
        };
        let message3 = Message {
            role: Role::User,
            content: "hello, I want you to help me".to_string(),
            name: None,
        };
        let messages = vec![message1, message2, message3];
        let messages_str = to_string(&messages).unwrap();
        println!("{}", messages_str);
    }
}
