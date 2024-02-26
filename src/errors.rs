use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErnieError {
    #[error("GetResponseError: {0}")]
    GetResponseError(String),
    #[error("InvokeError: {0}")]
    InvokeError(String),
    #[error("StreamError: {0}")]
    StreamError(String),
    #[error("GetAccessTokenError: {0}")]
    GetAccessTokenError(String),
    #[error("GenerateBodyError: {0}")]
    GenerateBodyError(String),
    #[error("RemoteAPIError: {0}")]
    RemoteAPIError(String),
    #[error("BuildUrlError: {0}")]
    BuildUrlError(#[from] url::ParseError),
}
