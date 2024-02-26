use erniebot_rs::text2image::{Size, Style, Text2ImageEndpoint, Text2ImageModel, Text2ImageOpt};
use erniebot_rs::utils::base64_to_image;
fn main() {
    let text2image = Text2ImageEndpoint::new(Text2ImageModel::StableDiffusionXL).unwrap();
    let prompt = "A beautiful sunset over the ocean".to_string();
    let mut options = Vec::new();
    options.push(Text2ImageOpt::Style(Style::DigitalArt));
    options.push(Text2ImageOpt::Size(Size::S1024x768));
    let text2image_response = text2image.invoke(prompt, options).unwrap();
    let image_results = text2image_response.get_image_results().unwrap();
    let mut index = 0;
    for image_string in image_results {
        let image = base64_to_image(image_string).unwrap();
        let filepath = format!("./tmp/image_{}.png", index);
        image.save(filepath).unwrap();
        index += 1;
    }
}
