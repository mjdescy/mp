//! Background Micropub publishing.
//!
//! The network request runs on a worker thread so the UI stays responsive;
//! the outcome travels back over an `mpsc` channel for the event loop to
//! collect without blocking.

use std::sync::mpsc;

use mplib::{MicropubService, Post, PostStatus, publish_post};

/// What the user chose to publish.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishTarget {
    Draft,
    Post,
}

/// Outcome of a background publish: the new post's result on success, or a
/// message to display on failure.
pub(crate) type PublishOutcome = Result<mplib::PostResult, String>;

/// A publish request running on a worker thread.
pub(crate) struct PublishJob {
    pub(crate) target: PublishTarget,
    pub(crate) receiver: mpsc::Receiver<PublishOutcome>,
}

/// Frames for the "Publishing..." spinner, advanced once per event-loop tick.
pub(crate) const SPINNER_FRAMES: [&str; 4] = ["|", "/", "-", "\\"];

/// Build a post from the editor body, extracting a leading markdown title
/// when the user opted into it.
pub(crate) fn build_post(body: String, target: PublishTarget, extract_title: bool) -> Post {
    let status = match target {
        PublishTarget::Draft => PostStatus::Draft,
        PublishTarget::Post => PostStatus::Published,
    };
    if extract_title {
        Post::from_body_with_title_extraction(body, status)
    } else {
        Post::from_body(body, status)
    }
}

/// Publish a post to completion: rebuild the service, run the async mplib
/// call on a fresh single-threaded runtime, and return the outcome.
/// Intended as the body of the background publish worker thread.
pub(crate) fn run_publish(api_url: String, auth_token: String, post: Post) -> PublishOutcome {
    let service = MicropubService::from_args(api_url, auth_token)
        .map_err(|e| format!("invalid Micropub configuration: {e}"))?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("failed to start publish runtime: {e}"))?;
    runtime
        .block_on(publish_post(post, &service))
        .map_err(|e| e.to_string())
}

/// Heuristic check for authentication failures in a publish error message.
/// mplib reports them as e.g. "API error: unauthorized - ...".
pub(crate) fn is_auth_failure(message: &str) -> bool {
    let message = message.to_lowercase();
    ["unauthorized", "invalid_token", "forbidden", "401", "403"]
        .iter()
        .any(|hint| message.contains(hint))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_failures_are_detected_for_config_reload() {
        assert!(is_auth_failure("API error: unauthorized - bad token"));
        assert!(is_auth_failure("API error: invalid_token - expired"));
        assert!(is_auth_failure("request failed with status 401"));
        assert!(!is_auth_failure(
            "API error: invalid_request - missing content"
        ));
        assert!(!is_auth_failure("network unreachable"));
    }

    #[test]
    fn build_post_extracts_title_only_when_enabled() {
        let body = "# Title\n\nBody text".to_string();

        let post = build_post(body.clone(), PublishTarget::Post, true);
        assert_eq!(post.title.as_deref(), Some("Title"));
        assert!(post.body.contains("Body text"));
        assert!(!post.body.contains("# Title"));

        let post = build_post(body, PublishTarget::Draft, false);
        assert_eq!(post.title, None);
    }
}
