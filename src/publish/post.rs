use std::time::Duration;
use reqwest::ClientBuilder;
use serde::Deserialize;
use serde_json::from_str;
use thiserror::Error;
use crate::configure::microblog_service::MicroblogService;
use crate::publish::post_status::PostStatus;
use crate::publish::post_result::PostResult;

const DEFAULT_TIMEOUT_SECS: u64 = 5;

#[derive(Error, Debug)]
pub enum PostError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("API error: {error} - {description}")]
    Api { error: String, description: String },
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Deserialize)]
struct ApiPostResponse {
    url: String,
    preview: String,
    edit: String,
}

#[derive(Deserialize)]
struct ApiPostError {
    error: String,
    error_description: String,
}

pub struct Post {
    pub content: String,
    pub status: PostStatus,
    pub microblog_service: MicroblogService,
}

impl Post {
    pub fn new(content: String, status: PostStatus, microblog_service: MicroblogService) -> Self {
        Post {
            content,
            status,
            microblog_service,
        }
    }

    pub async fn publish(&self) -> Result<PostResult, PostError> {
        let post_content = &self.content;
        let microblog_service = &self.microblog_service;
        let post_status = &self.status;

        if post_content.trim().is_empty() {
            return Err(PostError::InvalidInput("Post content cannot be empty".to_string()));
        }

        let client = Self::build_client()?;
        let response = Self::send_post_request(&client, post_content, microblog_service, post_status).await?;

        let status = response.status();
        let text: String = response.text().await?;

        if !status.is_success() {
            let api_error: ApiPostError = from_str(&text)?;
            return Err(PostError::Api {
                error: api_error.error,
                description: api_error.error_description,
            });
        }

        let api_response: ApiPostResponse = from_str(&text)?;

        Ok(PostResult {
            url: api_response.url,
            preview: api_response.preview,
            edit: api_response.edit,
            post_status: post_status.clone(),
        })
    }

    fn build_client() -> Result<reqwest::Client, PostError> {
        let timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECS);
        ClientBuilder::new()
            .timeout(timeout)
            .build()
            .map_err(PostError::Network)
    }

    async fn send_post_request(
        client: &reqwest::Client,
        post_content: &str,
        microblog_service: &MicroblogService,
        post_status: &PostStatus,
    ) -> Result<reqwest::Response, PostError> {
        let params = [
            ("h", "entry"),
            ("content", post_content),
            ("post-status", post_status.as_str()),
        ];

        client
            .post(&microblog_service.api_url)
            .form(&params)
            .bearer_auth(&microblog_service.auth_token)
            .send()
            .await
            .map_err(PostError::Network)
    }
}

