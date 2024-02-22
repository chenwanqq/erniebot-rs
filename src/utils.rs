use reqwest;
use std::{collections::HashMap, env::var};
use serde_json::Value;
use super::errors::GetAccessTokenError;
pub fn get_access_token() -> Result<String,GetAccessTokenError> {
    let url = "https://aip.baidubce.com/oauth/2.0/token";
    let ak = var("QIANFAN_AK").map_err(
        |_| GetAccessTokenError::Details("QIANFAN_AK is not set".to_string())
    )?;
    let sk = var("QIANFAN_SK").map_err(
        |_| GetAccessTokenError::Details("QIANFAN_SK is not set".to_string())
    )?;

    let client = reqwest::blocking::Client::new();
    let res: Value = client.post(url)
        .query(&[("grant_type", "client_credentials"), ("client_id", ak.as_str()), ("client_secret", sk.as_str())])
        .send()
        .map_err(|e| GetAccessTokenError::Details(e.to_string()))?
        .json()
        .map_err(|e| GetAccessTokenError::Details(e.to_string()))?;
    
    if let Some(error) = res.get("error") {
        let error_description = res.get("error_description").unwrap();
        Err(GetAccessTokenError::Details(format!("{}: {}", error, error_description)))
    } else {
        let access_token = res.get("access_token").unwrap().as_str().unwrap();
        Ok(access_token.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::get_access_token;
    /// before run the test, you should set the environment variables QIANFAN_AK and QIANFAN_SK
    #[test]
    fn test_get_access_token() {
        let access_token = get_access_token();
        println!("access_token: {:?}", access_token);
    }
}