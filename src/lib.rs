//! # mp - Micropub Publisher
//! 
//! A Rust library and CLI tool for publishing text-only blog posts to Micropub endpoints.
//! 
//! ## Library Usage
//! 
//! ```rust
//! use mp::{publish, AppConfig, PostStatus};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load configuration
//!     let app_config = AppConfig::load()?;
//!     
//!     // Publish a post
//!     let result = publish(app_config, "Hello, world!".to_string(), PostStatus::Published, false).await?;
//!     println!("Published: {}", result.url);
//!     
//!     Ok(())
//! }
//! ```

pub mod configure;
pub mod get_content;
pub mod publish;

// Re-export the main types and functions for library users
pub use configure::{AppConfig, MicroblogService};
pub use publish::{PostStatus, PostResult, PostError};

/// The main publish function that can be used by library consumers.
/// 
/// This function publishes content to a Micropub endpoint.
/// 
/// # Arguments
/// 
/// * `app_config` - The application configuration containing service details
/// * `post_content` - The content to publish
/// * `post_status` - Whether to publish as a draft or published post
/// * `quiet` - Whether to suppress output (only affects console output, not the result)
/// 
/// # Returns
/// 
/// Returns a `Result<PostResult, PostError>` containing the publication result or an error.
/// 
/// # Example
/// 
/// ```rust
/// use mp::{publish, AppConfig, PostStatus};
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let app_config = AppConfig::load()?;
///     let result = publish(app_config, "Hello, world!".to_string(), PostStatus::Published, false).await?;
///     println!("Published: {}", result.url);
///     Ok(())
/// }
/// ```
pub async fn publish(
    app_config: AppConfig, 
    post_content: String, 
    post_status: PostStatus, 
    quiet: bool
) -> Result<PostResult, PostError> {
    use publish::post::Post;
    
    if !quiet {
        let action = match post_status {
            PostStatus::Published => "Publishing post",
            PostStatus::Draft => "Publishing draft",
        };
        println!("{} with content:", action);
        println!("{}\n", post_content);
    }

    let post = Post::new(post_content, post_status, app_config.service);
    let post_result = post.publish().await;
    
    match post_result {
        Ok(result) => {
            if !quiet {
                println!("{}", result.as_string());
            }
            Ok(result)
        },
        Err(e) => Err(e)
    }
}

/// Convenience function to publish a post (non-draft)
pub async fn publish_post(
    app_config: AppConfig, 
    post_content: String, 
    quiet: bool
) -> Result<PostResult, PostError> {
    publish(app_config, post_content, PostStatus::Published, quiet).await
}

/// Convenience function to publish a draft
pub async fn publish_draft(
    app_config: AppConfig, 
    post_content: String, 
    quiet: bool
) -> Result<PostResult, PostError> {
    publish(app_config, post_content, PostStatus::Draft, quiet).await
}