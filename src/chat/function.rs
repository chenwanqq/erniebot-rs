use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Role;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
/// Definition of the function structure that some models(like Erniebot) can select and call.
pub struct Function {
    /// The name of the function.
    pub name: String,
    /// The description of the function.
    pub description: String,
    /// The format of parameters of the function, following the JSON schema format.
    pub parameters: RootSchema,
    /// The format of the response of the function, following the JSON schema format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<RootSchema>,
    /// The examples of the function. each instance of the outer vector represents a round of conversation, and each instance of the inner vector represents a message in the round of conversation. More details can be found in the example of chat_with_function.rs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<Vec<Example>>>,
}

/// Example of a message involved in a function calling process
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Example {
    /// Same as the role in Message, can be "user", "assistant", or "function".
    pub role: Role,
    /**Dialog content instructions:

    (1) When the current message contains a function_call and the role is "assistant", the message can be empty. However, in other scenarios, it cannot be empty.

    (2) The content corresponding to the last message cannot be a blank character, including spaces, "\n", "\r", "\f", etc.
    */
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The "author" of the message. the This member is required when the role value is "function", and in this case is should be the name in the function_call in the response content
    pub name: Option<String>,
    /// this is function calling result of last round of function call, serving as chat history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

/// This is function calling result of last round of function call
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct FunctionCall {
    /// name of a function
    pub name: String,
    /// arguments of a function call that LLM model outputs, following the JSON format.
    pub arguments: String,
    /// The thinking process of the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thoughts: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
/// In the context of function calls, prompt the large model to select a specific function (not mandatory). Note: The specified function name must exist within the list of functions.
pub struct ToolChoice {
    pub r#type: String, //only one valid value: "function"
    pub function: Value,
}

impl ToolChoice {
    pub fn new(function: Function) -> Self {
        Self {
            r#type: "function".to_string(),
            function: serde_json::json!(
                {
                    "name": function.name,
                }
            ),
        }
    }
    pub fn from_function_name(name: String) -> Self {
        Self {
            r#type: "function".to_string(),
            function: serde_json::json!(
                {
                    "name": name,
                }
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use schemars::{schema::RootSchema, schema_for, JsonSchema};
    use serde::{Deserialize, Serialize};
    #[test]
    fn test_schema() {
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
        #[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
        struct TestStruct {
            pub date: String,
            pub place: String,
        }
        let schema = schema_for!(TestStruct);
        println!("{:?}", serde_json::to_string(&schema).unwrap());
        let default_schema = RootSchema::default();
        println!("{:?}", serde_json::to_string(&default_schema).unwrap());
    }

    #[test]
    fn test_tool_choice() {
        use super::Function;
        let function = Function {
            name: "test".to_string(),
            description: "test".to_string(),
            ..Default::default()
        };
        let tool_choice = super::ToolChoice::new(function);
        println!("{:?}", serde_json::to_string(&tool_choice).unwrap());
    }
}
