use crate::publish::post_status::PostStatus;

pub struct PostResult {
    pub url: String,
    pub preview: String,
    pub edit: String,
    pub post_status: PostStatus,
}

impl PostResult {
    pub fn as_string(&self) -> String {
        format!(
            "{} successfully.\nURL: {}\nPreview: {}\nEdit: {}",
            self.post_status.action_description(), &self.url, &self.preview, &self.edit
        )
    }
}