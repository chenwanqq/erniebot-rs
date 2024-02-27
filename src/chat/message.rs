use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub enum Role {
    User,
    Assistant,
    Function,
}
/**  as metioned in <https://cloud.baidu.com/doc/WENXINWORKSHOP/s/jlil56u11>,
The "messages" member must not be empty. One member represents a single round of conversation, while multiple members represent multiple rounds of conversation. For example:
(1) Example with one member: "messages": [ {"role": "user", "content": "Hello"}]
```
    use erniebot_rs::chat::{Message, Role};
    let message1 = Message {
        role: Role::User,
        content: "hello, I'm a user".to_string(),
        name: None,
    };
    let messages = vec![message1];
```
(2) Example with three members:
```use erniebot_rs::chat::{Message, Role};
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
```
The last message is the current request information, while the previous messages are historical conversation information.

The number of members must be odd. The role values of the messages in the members are explained as follows: The role value of the message at odd positions must be either "user" or "function", while the role value of the message at even positions must be "assistant". The role of the first message cannot be "function". For example:

In the example, the role values of the messages are "user", "assistant", "user", "assistant", and "user" respectively. The role values of the messages at odd positions are "user", which means the role values of the 1st, 3rd, and 5th messages are "user". The role values at even positions are "assistant", which means the role values of the 2nd and 4th messages are "assistant".
*/
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub role: Role,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{Message, Role};
    use serde_json::{self, to_string};
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
