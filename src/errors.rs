use std::{error, fmt};
use thiserror::Error;
//TODO: use this error to replace all below
#[derive(Error, Debug)]
pub enum GetAccessTokenError {
    #[error("GetAccessTokenError: {0}")]
    Details(String),
}

#[derive(Error, Debug)]
pub enum GetResponseError {
    #[error("GetResponseError: {0}")]
    Details(String),
}

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("GetResponseError: {0}")]
    Response(#[from] GetResponseError),
    #[error("InvokeError: {0}")]
    InvokeError(String),
    #[error("StreamError: {0}")]
    StreamError(String),
    #[error("GetAccessTokenError: {0}")]
    GetAccessTokenError(#[from] GetAccessTokenError),
    #[error("BuildUrlError: {0}")]
    BuildUrlError(#[from] url::ParseError),
    #[error("GenerateBodyError: {0}")]
    GenerateBodyError(String),
    #[error("RemoteAPIError: {0}")]
    RemoteAPIError(String),
}
