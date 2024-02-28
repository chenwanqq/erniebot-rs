use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: RootSchema,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<RootSchema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<Vec<Example>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Example {
    pub role: String,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thoughts: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
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
