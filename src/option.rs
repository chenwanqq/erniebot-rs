use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub enum ResponseFormat {
    JsonObject,
    Text,
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

/* below are options for text2image */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, EnumString)]
pub enum Size {
    #[default]
    #[strum(serialize = "1024x1024")]
    S1024x1024,
    #[strum(serialize = "768x768")]
    S768x768,
    #[strum(serialize = "768x1024")]
    S768x1024,
    #[strum(serialize = "1024x768")]
    S1024x768,
    #[strum(serialize = "576x1024")]
    S576x1024,
    #[strum(serialize = "1024x576")]
    S1024x576,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, EnumString)]
pub enum SamplerIndex {
    #[default]
    #[strum(serialize = "Euler a")]
    EulerA,
    #[strum(serialize = "Euler")]
    Euler,
    #[strum(serialize = "DPM++ 2M")]
    DPMpp2M,
    #[strum(serialize = "DPM++ 2M Karras")]
    DPMpp2MKarras,
    #[strum(serialize = "LMS Karras")]
    LMSKarras,
    #[strum(serialize = "DPM++ SDE")]
    DPMppSDE,
    #[strum(serialize = "DPM++ SDE Karras")]
    DPMppSDEKarras,
    #[strum(serialize = "DPM2 a Karras")]
    DPM2AKarras,
    #[strum(serialize = "Heun")]
    Heun,
    #[strum(serialize = "DPM++ 2M SDE")]
    DPMpp2MSDE,
    #[strum(serialize = "DPM++ 2M SDE Karras")]
    DPMpp2MSDEKarras,
    #[strum(serialize = "DPM2")]
    DPM2,
    #[strum(serialize = "DPM2 Karras")]
    DPM2Karras,
    #[strum(serialize = "DPM2 a")]
    DPM2A,
    #[strum(serialize = "LMS")]
    LMS,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub enum Text2ImageOpt {
    Prompt(String),
    NegativePrompt(String),
    Size(Size),
    N(u32),     //1-4
    Steps(u32), //10-50
}
