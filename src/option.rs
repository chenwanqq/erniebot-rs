use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Options {
    opts: Vec<Opt>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
pub enum Opt {
    Temperature(f32),
    TopP(f32),
    PenaltyScore(f32),
    Stream(bool),
    System(String),
    Stop(Vec<String>),
    DisableSearch(bool),
    EnableCitation(bool),
    MaxOutputTokens(u32),
    ResponseFormat(ResponseFormat),
    UserId(String),
}
