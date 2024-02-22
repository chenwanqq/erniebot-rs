use std::{error,fmt};
//TODO: use this error to replace all below
#[derive(Debug)]
pub struct GetAccessTokenError {
    details: String
}

impl fmt::Display for GetAccessTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl GetAccessTokenError {
    pub fn new(msg: &str) -> GetAccessTokenError {
        GetAccessTokenError{details: msg.to_string()}
    }
}

impl error::Error for GetAccessTokenError {}

pub struct GetResultError {
    details: String
}

impl fmt::Display for GetResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}




