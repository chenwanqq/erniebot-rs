use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Default, Clone, Serialize, Deserialize, EnumString, Display, PartialEq, Eq)]
#[non_exhaustive]
pub enum EmbeddingModel {
    #[default]
    #[strum(serialize = "embedding-v1")]
    #[serde(rename = "embedding-v1")]
    EmbeddingV1,
    #[strum(serialize = "bge_large_zh")]
    #[serde(rename = "bge_large_zh")]
    BgeLargeZh,
    #[strum(serialize = "bge_large_en")]
    #[serde(rename = "bge_large_en")]
    BgeLagreEn,
    #[strum(serialize = "tao_8k")]
    #[serde(rename = "tao_8k")]
    Tao8k,
}

#[cfg(test)]
mod tests {
    use super::EmbeddingModel;
    use std::str::FromStr;
    #[test]
    fn test_embedding_model_to_string() {
        assert_eq!(EmbeddingModel::EmbeddingV1.to_string(), "embedding-v1");
        assert_eq!(EmbeddingModel::BgeLargeZh.to_string(), "bge_large_zh");
        assert_eq!(EmbeddingModel::BgeLagreEn.to_string(), "bge_large_en");
        assert_eq!(EmbeddingModel::Tao8k.to_string(), "tao_8k");
    }

    #[test]
    fn test_embedding_model_from_str() {
        assert_eq!(
            EmbeddingModel::from_str("embedding-v1").unwrap(),
            EmbeddingModel::EmbeddingV1
        );
        assert_eq!(
            EmbeddingModel::from_str("bge_large_zh").unwrap(),
            EmbeddingModel::BgeLargeZh
        );
        assert_eq!(
            EmbeddingModel::from_str("bge_large_en").unwrap(),
            EmbeddingModel::BgeLagreEn
        );
        assert_eq!(
            EmbeddingModel::from_str("tao_8k").unwrap(),
            EmbeddingModel::Tao8k
        );
    }
}
