use std::io;
use serde::{Deserialize, Serialize};
use crate::configure::user_input::{get_user_input};

#[derive(Serialize, Deserialize)]
pub struct MicroblogService {
    pub api_url: String,
    pub auth_token: String,
}
 
impl MicroblogService {
    pub fn new(api_url: String, auth_token: String) -> Self {
        MicroblogService { api_url, auth_token }
    }

    pub fn from_args(api_url: String, auth_token: String) -> io::Result<Self> {
        if api_url.is_empty() || auth_token.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "API URL and authentication token must not be empty",
            ));
        }
        Ok(MicroblogService::new(api_url, auth_token))
    }

    pub fn from_user_input() -> io::Result<Self> {        
        println!("=============================================");
        println!("Define the micoblogging service to publish to");
        println!("=============================================");
        let api_url = get_user_input("[Step 1 of 2] Enter API URL");
        let auth_token = get_user_input("[Step 2 of 2] Enter Authentication Token");

        Self::from_args(api_url, auth_token)
    }
}
