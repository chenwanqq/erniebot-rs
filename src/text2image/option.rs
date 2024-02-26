use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/* below are options for text2image */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Display, EnumString)]
pub enum Size {
    #[default]
    #[strum(serialize = "1024x1024")]
    #[serde(rename = "1024x1024")]
    S1024x1024,
    #[strum(serialize = "768x768")]
    #[serde(rename = "768x768")]
    S768x768,
    #[strum(serialize = "768x1024")]
    #[serde(rename = "768x1024")]
    S768x1024,
    #[strum(serialize = "1024x768")]
    #[serde(rename = "1024x768")]
    S1024x768,
    #[strum(serialize = "576x1024")]
    #[serde(rename = "576x1024")]
    S576x1024,
    #[strum(serialize = "1024x576")]
    #[serde(rename = "1024x576")]
    S1024x576,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, EnumString)]
pub enum SamplerIndex {
    #[default]
    #[strum(serialize = "Euler a")]
    #[serde(rename = "Euler a")]
    EulerA,
    #[strum(serialize = "Euler")]
    #[serde(rename = "Euler")]
    Euler,
    #[strum(serialize = "DPM++ 2M")]
    #[serde(rename = "DPM++ 2M")]
    DPMpp2M,
    #[strum(serialize = "DPM++ 2M Karras")]
    #[serde(rename = "DPM++ 2M Karras")]
    DPMpp2MKarras,
    #[strum(serialize = "LMS Karras")]
    #[serde(rename = "LMS Karras")]
    LMSKarras,
    #[strum(serialize = "DPM++ SDE")]
    #[serde(rename = "DPM++ SDE")]
    DPMppSDE,
    #[strum(serialize = "DPM++ SDE Karras")]
    #[serde(rename = "DPM++ SDE Karras")]
    DPMppSDEKarras,
    #[strum(serialize = "DPM2 a Karras")]
    #[serde(rename = "DPM2 a Karras")]
    DPM2AKarras,
    #[strum(serialize = "Heun")]
    #[serde(rename = "Heun")]
    Heun,
    #[strum(serialize = "DPM++ 2M SDE")]
    #[serde(rename = "DPM++ 2M SDE")]
    DPMpp2MSDE,
    #[strum(serialize = "DPM++ 2M SDE Karras")]
    #[serde(rename = "DPM++ 2M SDE Karras")]
    DPMpp2MSDEKarras,
    #[strum(serialize = "DPM2")]
    #[serde(rename = "DPM2")]
    DPM2,
    #[strum(serialize = "DPM2 Karras")]
    #[serde(rename = "DPM2 Karras")]
    DPM2Karras,
    #[strum(serialize = "DPM2 a")]
    #[serde(rename = "DPM2 a")]
    DPM2A,
    #[strum(serialize = "LMS")]
    #[serde(rename = "LMS")]
    LMS,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, EnumString)]
pub enum Style {
    #[default]
    #[strum(serialize = "Base")]
    #[serde(rename = "Base")]
    Base,
    #[strum(serialize = "3D Model")]
    #[serde(rename = "3D Model")]
    Model3D,
    #[strum(serialize = "Analog Film")]
    #[serde(rename = "Analog Film")]
    AnalogFilm,
    #[strum(serialize = "Anime")]
    #[serde(rename = "Anime")]
    Anime,
    #[strum(serialize = "Cinematic")]
    #[serde(rename = "Cinematic")]
    Cinematic,
    #[strum(serialize = "Comic Book")]
    #[serde(rename = "Comic Book")]
    ComicBook,
    #[strum(serialize = "Craft Clay")]
    #[serde(rename = "Craft Clay")]
    CraftClay,
    #[strum(serialize = "Digital Art")]
    #[serde(rename = "Digital Art")]
    DigitalArt,
    #[strum(serialize = "Enhance")]
    #[serde(rename = "Enhance")]
    Enhance,
    #[strum(serialize = "Fantasy Art")]
    #[serde(rename = "Fantasy Art")]
    FantasyArt,
    #[strum(serialize = "Isometric")]
    #[serde(rename = "Isometric")]
    Isometric,
    #[strum(serialize = "Line Art")]
    #[serde(rename = "Line Art")]
    LineArt,
    #[strum(serialize = "Lowpoly")]
    #[serde(rename = "Lowpoly")]
    Lowpoly,
    #[strum(serialize = "Neonpunk")]
    #[serde(rename = "Neonpunk")]
    Neonpunk,
    #[strum(serialize = "Origami")]
    #[serde(rename = "Origami")]
    Origami,
    #[strum(serialize = "Photographic")]
    #[serde(rename = "Photographic")]
    Photographic,
    #[strum(serialize = "Pixel Art")]
    #[serde(rename = "Pixel Art")]
    PixelArt,
    #[strum(serialize = "Texture")]
    #[serde(rename = "Texture")]
    Texture,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumString)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub enum Text2ImageOpt {
    NegativePrompt(String),
    Size(Size),
    N(u32),     //1-4
    Steps(u32), //10-50
    SamplerIndex(SamplerIndex),
    Seed(u32),     //0, 4294967295]
    CfgScale(f32), //0-30
    Style(Style),
    UserId(String),
}

#[cfg(test)]
mod tests {
    use super::Size;
    #[test]
    fn test_size() {
        let size = Size::S1024x1024;
        assert_eq!(size.to_string(), "1024x1024");
    }
}
