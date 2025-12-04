//! # mplib - Micropub Publisher Library
//! 
//! A Rust library for publishing text-only blog posts to Micropub endpoints.
//! 
//! ## Example
//! 
//!! ```rust
//! use mplib::{MicroblogService, Post, publish_post};
//! 
//! fn main() {
//!     let service = MicroblogService::new("https://example.com/micropub", "your_access_token");
//!     let post = Post::new("Hello, world!", None);
//!     match publish_post(&service, &post) {
//!         Ok(result) => println!("Post published successfully: {:?}", result),
//!         Err(e) => eprintln!("Failed to publish post: {}", e),
//!     }
//! }
//! ```

mod microblog_service;
mod post;
mod post_result;
mod post_status;
mod publish;

// Re-export the main types and functions for library users
pub use crate::microblog_service::MicroblogService;
pub use crate::post::Post;
pub use crate::post_result::PostResult;
pub use crate::post_status::PostStatus;
pub use crate::publish::publish_post;