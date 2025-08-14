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

    pub fn is_draft(&self) -> bool {
        matches!(self, PostStatus::Draft)
    }
}
