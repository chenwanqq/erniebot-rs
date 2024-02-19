use url::{Url,ParseError};

static CHAT_API_URL: &'static str =
    "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/";

fn build_url(model: &str) -> Result<Url, ParseError> {
    let base = Url::parse(CHAT_API_URL).expect("url is invalid");
    let joined = base.join(model)?;
    Ok(joined)
}



#[cfg(test)]
mod tests {
    use super::build_url;
    use url::Url;

    #[test]
    fn test_build_url() {
        let url = build_url("completions_pro").unwrap();
        assert_eq!(url, Url::parse("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/completions_pro").unwrap());
    }
}