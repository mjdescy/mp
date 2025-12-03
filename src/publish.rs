pub mod post;
pub mod post_status;
pub mod post_result;

use std::time::Duration;

use reqwest::ClientBuilder;
use serde_json::from_str;
use serde::Deserialize;
use thiserror::Error;

pub use crate::configuration::microblog_service::MicroblogService;
use crate::publish::post::Post;

pub use crate::publish::post_result::PostResult;

const DEFAULT_TIMEOUT_SECS: u64 = 5;

/// Publish a Post via a Micropub service.
pub async fn publish_post(
    post: Post,
    service: &MicroblogService,
) -> Result<PostResult, PostError> {
    if post.is_empty() {
        return Err(PostError::InvalidInput("Post content cannot be empty".to_string()));
    }

    let client = build_client()?;
    let post_status = post.status.clone();
    let response = send_post_request(
        &client, post, &service).await?;

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
        post_status,
    })
}

/// Build an HTTP client with a default timeout.
fn build_client() -> Result<reqwest::Client, PostError> {
    let timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECS);
    ClientBuilder::new()
        .timeout(timeout)
        .build()
        .map_err(PostError::Network)
}

/// Send a POST request to the Micropub endpoint with the given post data.
async fn send_post_request(
    client: &reqwest::Client,
    post: Post,
    microblog_service: &MicroblogService,
) -> Result<reqwest::Response, PostError> {
    // Prepare form parameters. If post has a title, include it. Otherwise, omit it.
    let params = [
        ("h", "entry"),
        ("content", post.body.as_str()),
        ("post-status", post.status.as_str()),
    ]
        .into_iter()
        .chain(post.title.as_ref().map(|t| ("name", t.as_str())))
        .collect::<Vec<_>>();

    // Send the POST request.
    client
        .post(&microblog_service.api_url)
        .form(&params)
        .bearer_auth(&microblog_service.auth_token)
        .send()
        .await
        .map_err(PostError::Network)
}

/// Represents errors that can occur while publishing a post.
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

/// Represents the expected response from the Micropub API upon successful post creation.
#[derive(Deserialize)]
struct ApiPostResponse {
    url: String,
    preview: String,
    edit: String,
}

/// Represents the expected error response from the Micropub API upon a failed post creation.
#[derive(Deserialize)]
struct ApiPostError {
    error: String,
    error_description: String,
}

