use std::io;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MicropubService {
    pub api_url: String,
    pub auth_token: String,
}
 
impl MicropubService {
    pub fn new(api_url: String, auth_token: String) -> Self {
        MicropubService { api_url, auth_token }
    }

    pub fn from_args(api_url: String, auth_token: String) -> io::Result<Self> {
        if api_url.is_empty() || auth_token.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "API URL and authentication token must not be empty",
            ));
        }
        Ok(MicropubService::new(api_url, auth_token))
    }
}