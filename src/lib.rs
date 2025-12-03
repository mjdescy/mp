//! # mp - Micropub Publisher
//! 
//! A Rust library and CLI tool for publishing text-only blog posts to Micropub endpoints.
//! 
//! ## Library Usage
//! 
//! ```no_run
//! use mp::{publish, MicroblogService, PostStatus};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a service configuration
//!     let service = MicroblogService::new(
//!         "https://micro.blog/micropub".to_string(),
//!         "your-auth-token".to_string()
//!     );
//!     
//!     // PublCreate a post to publish
//!     let post - publish::post::Post::from_body_and_title(
//!         "Hello, world!".to_string(),
//!         "My First Post".to_string(),
//!         PostStatus::Published
//!     );
//!     
//!     // Publish the post
//!     let result = publish_post(
//!     post,
//!    service
//!     ).await?;
//!     println!("Published: {}", result.url);
//!     
//!     Ok(())
//! }
//! ```

pub mod configuration;
mod publish;
pub mod cli;

// Re-export the main types and functions for library users
pub use crate::configuration::microblog_service::MicroblogService;
pub use crate::publish::{PostResult, PostError, publish_post};
pub use crate::publish::post::Post;
pub use crate::publish::post_status::PostStatus;
