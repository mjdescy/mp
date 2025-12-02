
/// Represents the status of a post, either published or draft.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostStatus {
    Published,
    Draft,
}

impl PostStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PostStatus::Published => "published",
            PostStatus::Draft => "draft",
        }
    }

    pub fn action_description(&self) -> &str {
        match self {
            PostStatus::Published => "Post published",
            PostStatus::Draft => "Draft created",
        }
    }
}
