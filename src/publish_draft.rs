mod post_status;

use std::time::Duration;
use reqwest::ClientBuilder;
use serde::Deserialize;
use serde_json::from_str;
use crate::configure::microblog_service::MicroblogService;
use crate::publish_draft::post_status::PostStatus;

#[derive(Deserialize)]
pub struct PostResponse {
    url: String,
    preview: String,
    edit: String,
}

#[derive(Deserialize)]
pub struct PostError {
    error: String,
    error_description: String,
}

#[allow(dead_code)]
pub async fn create_post(post_content: &str, microblog_service: &MicroblogService, post_status: PostStatus) -> String {
    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new()
        .timeout(timeout)
        .build()
        .expect("cannot build client");

    let params = [
        ("h", "entry"),
        ("content", &post_content),
        ("post-status", post_status.as_str()),
    ];

    let response = client
        .post(&microblog_service.api_url)
        .form(&params)
        .bearer_auth(&microblog_service.auth_token)
        .send()
        .await
        .expect("error during API call");

    let status = response.status();
    let text = &response.text().await.expect("error parsing response");

    if !status.is_success() {
        let post_error: PostError = from_str(&text).expect("Failed to parse error response");
        let error = format!(
            "Failed to publish post ({}).\n{}",
            post_error.error, post_error.error_description
        );
        return error;
    }

    let post: PostResponse = from_str(&text).expect("Failed to parse response");

    let action_description = if post_status.is_draft() {
        "Draft created"
    } else {
        "Post published"
    };

    format!(
        "{} successfully.\nURL: {}\nPreview: {}\nEdit: {}",
        action_description, post.url, post.preview, post.edit
    )
}
