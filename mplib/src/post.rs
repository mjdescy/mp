use crate::post_status::PostStatus;

/// Represents a post to be published.
pub struct Post {
    pub body: String,
    pub title: Option<String>,
    pub status: PostStatus,
}

impl Post {
    /// Create a new Post with the given body and status, without a title.
    pub fn from_body(body: String, status: PostStatus) -> Self {
        Post {
            body,
            title: None,
            status,
        }
    }

    /// Create a new Post with the given body, title, and status.
    pub fn from_body_and_title(body: String, title: String, status: PostStatus) -> Self {
        Post {
            body,
            title: Some(title),
            status,
        }
    }

    /// Create a new Post by extracting the title from the body if it starts with a markdown
    /// level 1 header (i.e., a line starting with '# ').
    pub fn from_body_with_title_extraction(body: String, status: PostStatus) -> Self {
        let (body, title) = Self::separate_title_from_body(body);
        Post {
            body,
            title,
            status,
        }
    }

    /// Separate the title from the body if the body starts with a markdown level 1 header.
    /// Returns a tuple of the remaining body and an optional extracted title.
    fn separate_title_from_body(body: String) -> (String, Option<String>) {
        // The title will be the first line of the body, if the first line
        // is a markdown level 1 header (starts with '# ').
        // If the title is extracted, remove it from the body along with any blank lines
        // between the title and the rest of the body.
        let mut lines = body.lines();
        if let Some(first_line) = lines.next() {
            if first_line.starts_with("# ") {
                let extracted_title = first_line[2..].trim().to_string();
                let remaining_body: String = lines
                    .skip_while(|line| line.trim().is_empty())
                    .collect::<Vec<&str>>()
                    .join("\n");
                (remaining_body, Some(extracted_title))
            } else {
                (body, None)
            }
        } else {
            (body, None)
        }
    }

    /// Check if the post body is empty or consists only of whitespace.
    pub fn is_empty(&self) -> bool {
        self.body.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty_with_empty_string() {
        let post = Post::from_body("   \n\t  ".to_string(), PostStatus::Draft);
        assert!(post.is_empty());
    }

    #[test]
    fn test_is_empty_with_whitespace_only() {
        let post = Post::from_body("   \n\t  ".to_string(), PostStatus::Draft);
        assert!(post.is_empty());
    }

    #[test]
    fn test_is_empty_with_content() {
        let post = Post::from_body("Some content".to_string(), PostStatus::Draft);
        assert!(!post.is_empty());
    }

    #[test]
    fn test_from_body_creates_post_without_title() {
        let post = Post::from_body("Body text".to_string(), PostStatus::Published);
        assert_eq!(post.body, "Body text");
        assert_eq!(post.title, None);
        assert_eq!(post.status, PostStatus::Published);
    }

    #[test]
    fn test_from_body_and_title_creates_post_with_title() {
        let post =
            Post::from_body_and_title("Body".to_string(), "Title".to_string(), PostStatus::Draft);
        assert_eq!(post.body, "Body");
        assert_eq!(post.title, Some("Title".to_string()));
        assert_eq!(post.status, PostStatus::Draft);
    }

    #[test]
    fn test_from_body_with_title_extraction_extracts_title() {
        let post = Post::from_body_with_title_extraction(
            "# Title\n\nBody".to_string(),
            PostStatus::Published,
        );
        assert_eq!(post.title, Some("Title".to_string()));
        assert_eq!(post.body, "Body");
        assert_eq!(post.status, PostStatus::Published);
    }

    #[test]
    fn test_from_body_with_title_extraction_no_title() {
        let post =
            Post::from_body_with_title_extraction("Just body".to_string(), PostStatus::Draft);
        assert_eq!(post.title, None);
        assert_eq!(post.body, "Just body");
    }

    #[test]
    fn test_separate_title_from_body_heading_1_in_first_line() {
        let body = "# My Title\nThis is the body".to_string();
        let (remaining_body, title) = Post::separate_title_from_body(body);
        assert_eq!(title, Some("My Title".to_string()));
        assert_eq!(remaining_body, "This is the body");
    }

    #[test]
    fn test_separate_title_from_body_heading_2_in_first_line() {
        let body = "## My Title\nThis is the body".to_string();
        let (remaining_body, title) = Post::separate_title_from_body(body);
        assert_eq!(title, None);
        assert_eq!(remaining_body, "## My Title\nThis is the body");
    }

    #[test]
    fn test_separate_title_from_body_blank_line_then_heading_1() {
        let body = "\n# My Title\nThis is the body".to_string();
        let (remaining_body, title) = Post::separate_title_from_body(body);
        assert_eq!(title, None);
        assert_eq!(remaining_body, "\n# My Title\nThis is the body");
    }

    #[test]
    fn test_separate_title_from_body_no_blank_line_between() {
        let body = "# My Title\nThis is the body".to_string();
        let (remaining_body, title) = Post::separate_title_from_body(body);
        assert_eq!(title, Some("My Title".to_string()));
        assert_eq!(remaining_body, "This is the body");
    }

    #[test]
    fn test_separate_title_from_body_one_blank_line_between() {
        let body = "# My Title\n\nThis is the body".to_string();
        let (remaining_body, title) = Post::separate_title_from_body(body);
        assert_eq!(title, Some("My Title".to_string()));
        assert_eq!(remaining_body, "This is the body");
    }

    #[test]
    fn test_separate_title_from_body_multiple_blank_lines_between() {
        let body = "# My Title\n\n\n\nThis is the body".to_string();
        let (remaining_body, title) = Post::separate_title_from_body(body);
        assert_eq!(title, Some("My Title".to_string()));
        assert_eq!(remaining_body, "This is the body");
    }
}
