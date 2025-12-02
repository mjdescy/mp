use crate::publish::post_status::PostStatus;

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