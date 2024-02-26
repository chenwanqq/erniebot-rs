use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Default, Clone, Serialize, Deserialize, EnumString, Display, PartialEq, Eq)]
#[non_exhaustive]
pub enum Text2ImageModel {
    #[default]
    #[strum(serialize = "sd_xl")]
    StableDiffusionXL,
}

#[cfg(test)]
mod tests {
    use super::Text2ImageModel;
    use std::str::FromStr;
    #[test]
    fn test_text2image_model_to_string() {
        assert_eq!(Text2ImageModel::StableDiffusionXL.to_string(), "sd_xl");
    }

    #[test]
    fn test_text2image_model_from_str() {
        assert_eq!(
            Text2ImageModel::from_str("sd_xl").unwrap(),
            Text2ImageModel::StableDiffusionXL
        );
    }
}
