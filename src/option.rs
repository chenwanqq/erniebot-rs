use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub enum ResponseFormat {
    JsonObject,
    Text,
}
impl ToString for ResponseFormat {
    fn to_string(&self) -> String {
        match self {
            ResponseFormat::JsonObject => "json_object".to_string(),
            ResponseFormat::Text => "text".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub enum ChatOpt {
    Temperature(f32),
    TopP(f32),
    TopK(u32),
    PenaltyScore(f32),
    System(String),
    Stop(Vec<String>),
    DisableSearch(bool),
    EnableCitation(bool),
    MaxOutputTokens(u32),
    ResponseFormat(ResponseFormat),
    UserId(String),
}
