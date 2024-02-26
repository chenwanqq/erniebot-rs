use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Default, Clone, Serialize, Deserialize, EnumString, Display, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChatModel {
    #[default]
    #[strum(serialize = "eb-instant")]
    #[serde(rename = "eb-instant")]
    ErnieBotTurbo,
    #[strum(serialize = "completions")]
    #[serde(rename = "completions")]
    ErnieBot,
    #[strum(serialize = "completions_pro")]
    #[serde(rename = "completions_pro")]
    Ernie40,
}

#[cfg(test)]
mod tests {
    use super::ChatModel;
    use std::str::FromStr;
    #[test]
    fn test_chat_model_to_string() {
        assert_eq!(ChatModel::ErnieBotTurbo.to_string(), "eb-instant");
        assert_eq!(ChatModel::ErnieBot.to_string(), "completions");
        assert_eq!(ChatModel::Ernie40.to_string(), "completions_pro");
    }

    #[test]
    fn test_chat_model_from_str() {
        assert_eq!(
            ChatModel::from_str("eb-instant").unwrap(),
            ChatModel::ErnieBotTurbo
        );
        assert_eq!(
            ChatModel::from_str("completions").unwrap(),
            ChatModel::ErnieBot
        );
        assert_eq!(
            ChatModel::from_str("completions_pro").unwrap(),
            ChatModel::Ernie40
        );
    }
}
