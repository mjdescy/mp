use crate::publish::post_status::PostStatus;

/// Represents a micropub endpoint's response to a request to publish a post.
pub struct PostResult {
    pub url: String,
    pub preview: String,
    pub edit: String,
    pub post_status: PostStatus,
}

impl PostResult {
    /// Format the PostResult as a human-readable string.
    pub fn as_string(&self) -> String {
        format!(
            "{} successfully.\n\nURL:     {}\nPreview: {}\nEdit:    {}",
            self.post_status.action_description(), &self.url, &self.preview, &self.edit
        )
    }
}