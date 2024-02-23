use super::errors::ErnieError;
use super::model::EmbeddingModel;
use super::response::Response;
use super::utils::{build_url, get_access_token};
use json_value_merge::Merge;
use url::Url;

static EMBEDDING_BASE_URL: &'static str =
    "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/embeddings/";

struct Embedding {
    url: Url,
    access_token: String,
}

impl Embedding {
    pub fn new(model: EmbeddingModel) -> Result<Self, ErnieError> {
        Ok(Embedding {
            url: build_url(EMBEDDING_BASE_URL, model.to_string().as_str())?,
            access_token: get_access_token()?,
        })
    }

    pub fn invoke(
        &self,
        input: Vec<String>,
        user_id: Option<String>,
    ) -> Result<Response, ErnieError> {
        let mut body = serde_json::json!({
            "input": input,
        });
        if let Some(user_id) = user_id {
            body.merge(&serde_json::json!({"user_id": user_id}));
        }
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(self.url.as_str())
            .query(&[("access_token", self.access_token.as_str())])
            .json(&body)
            .send()
            .map_err(|e| ErnieError::InvokeError(e.to_string()))?
            .json()
            .map_err(|e| ErnieError::InvokeError(e.to_string()))?;
        Ok(Response::new(res))
    }
}

#[cfg(test)]
mod tests {
    use super::super::model::EmbeddingModel;
    use super::Embedding;
    #[test]
    fn test_embedding() {
        let embedding = Embedding::new(EmbeddingModel::EmbeddingV1).unwrap();
        let input = vec![
            "你好".to_string(),
            "你叫什么名字".to_string(),
            "你是谁".to_string(),
        ];
        let response = embedding.invoke(input, None).unwrap();
        let embedding_results = response.get_embedding_results().unwrap();
        println!("{},{}", embedding_results.len(), embedding_results[0].len());
    }
}
