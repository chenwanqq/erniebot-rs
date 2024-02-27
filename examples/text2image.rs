use erniebot_rs::text2image::{Size, Style, Text2ImageEndpoint, Text2ImageModel, Text2ImageOpt};
use erniebot_rs::utils::base64_to_image;
fn main() {
    let text2image = Text2ImageEndpoint::new(Text2ImageModel::StableDiffusionXL).unwrap();
    let prompt = "A beautiful sunset over the ocean".to_string();
    let options = vec![
        Text2ImageOpt::Style(Style::DigitalArt),
        Text2ImageOpt::Size(Size::S1024x768),
    ];
    let text2image_response = text2image.invoke(prompt, options).unwrap();
    let image_results = text2image_response.get_image_results().unwrap();
    for (index, image_string) in image_results.into_iter().enumerate() {
        let image = base64_to_image(image_string).unwrap();
        let filepath = format!("./tmp/image_{}.png", index);
        image.save(filepath).unwrap();
    }
}
