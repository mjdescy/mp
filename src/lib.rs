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
//!     // Publish a post
//!     let result = publish(service, "Hello, world!".to_string(), None, PostStatus::Published, false).await?;
//!     println!("Published: {}", result.url);
//!     
//!     Ok(())
//! }
//! ```

pub mod cli;
pub mod configure;
pub mod publish;

// Re-export the main types and functions for library users
pub use configure::MicroblogService;
pub use publish::{PostStatus, PostResult, PostError};
pub use publish::publish_post;
