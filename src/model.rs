use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Default, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChatModel {
    #[default]
    #[strum(serialize = "eb-instant")]
    ErnieBotTurbo,
    #[strum(serialize = "completions")]
    ErnieBot,
    #[strum(serialize = "completions_pro")]
    Ernie40,
    #[strum(default)]
    Other(String),
}

impl ToString for ChatModel {
    fn to_string(&self) -> String {
        match self {
            ChatModel::ErnieBotTurbo => "eb-instant".to_string(),
            ChatModel::ErnieBot => "completions".to_string(),
            ChatModel::Ernie40 => "completions_pro".to_string(),
            ChatModel::Other(model) => model.to_string(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[non_exhaustive]
pub enum EmbeddingModel {
    #[default]
    #[strum(serialize = "embedding-v1")]
    EmbeddingV1,
    #[strum(serialize = "bge_large_zh")]
    BgeLargeZh,
    #[strum(serialize = "bge_large_en")]
    BgeLagreEn,
    #[strum(serialize = "tao_8k")]
    Tao8k,
    #[strum(default)]
    Other(String),
}

impl ToString for EmbeddingModel {
    fn to_string(&self) -> String {
        match self {
            EmbeddingModel::EmbeddingV1 => "embedding-v1".to_string(),
            EmbeddingModel::BgeLargeZh => "bge_large_zh".to_string(),
            EmbeddingModel::BgeLagreEn => "bge_large_en".to_string(),
            EmbeddingModel::Tao8k => "tao_8k".to_string(),
            EmbeddingModel::Other(model) => model.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ChatModel, EmbeddingModel};
    use std::str::FromStr;

    #[test]
    fn test_from_str() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(ChatModel::from_str("eb-instant")?, ChatModel::ErnieBotTurbo);
        assert_eq!(ChatModel::from_str("completions")?, ChatModel::ErnieBot);
        assert_eq!(ChatModel::from_str("completions_pro")?, ChatModel::Ernie40);
        assert_eq!(
            ChatModel::from_str("custom ChatModel")?,
            ChatModel::Other("custom ChatModel".to_string())
        );
        assert_eq!(
            EmbeddingModel::from_str("embedding-v1")?,
            EmbeddingModel::EmbeddingV1
        );
        assert_eq!(
            EmbeddingModel::from_str("bge_large_zh")?,
            EmbeddingModel::BgeLargeZh
        );
        assert_eq!(
            EmbeddingModel::from_str("bge_large_en")?,
            EmbeddingModel::BgeLagreEn
        );
        assert_eq!(EmbeddingModel::from_str("tao_8k")?, EmbeddingModel::Tao8k);
        assert_eq!(
            EmbeddingModel::from_str("custom EmbeddingModel")?,
            EmbeddingModel::Other("custom EmbeddingModel".to_string())
        );
        Ok(())
    }

    #[test]
    fn test_to_string() {
        assert_eq!(ChatModel::ErnieBotTurbo.to_string(), "eb-instant");
        assert_eq!(ChatModel::ErnieBot.to_string(), "completions");
        assert_eq!(ChatModel::Ernie40.to_string(), "completions_pro");
        assert_eq!(
            ChatModel::Other("custom ChatModel".to_string()).to_string(),
            "custom ChatModel"
        );
        assert_eq!(EmbeddingModel::EmbeddingV1.to_string(), "embedding-v1");
        assert_eq!(EmbeddingModel::BgeLargeZh.to_string(), "bge_large_zh");
        assert_eq!(EmbeddingModel::BgeLagreEn.to_string(), "bge_large_en");
        assert_eq!(EmbeddingModel::Tao8k.to_string(), "tao_8k");
        assert_eq!(
            EmbeddingModel::Other("custom EmbeddingModel".to_string()).to_string(),
            "custom EmbeddingModel"
        );
    }
}
