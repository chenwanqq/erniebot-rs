use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Default, Clone, Serialize, Deserialize, EnumString, Display, PartialEq, Eq)]
#[non_exhaustive]
pub enum RerankerModel {
    #[default]
    #[strum(serialize = "bce_reranker_base")]
    #[serde(rename = "bce_reranker_base")]
    BceRerankerBaseV1,
}
