mod post;
mod post_result;
mod post_status;

use crate::publish::post::Post;
use crate::publish::post_result::PostResult;  // Add this import
pub use crate::publish::post_status::PostStatus;
use crate::configure::AppConfig;

pub async fn handle_draft_verb(app_config: AppConfig, post_content: String) -> Result<PostResult, post::PostError> {
    let post_status = PostStatus::Draft;

    publish(app_config, post_content, post_status).await
}

pub async fn handle_post_verb(app_config: AppConfig, post_content: String) -> Result<PostResult, post::PostError> {
    let post_status = PostStatus::Published;
    
    publish(app_config, post_content, post_status).await
}

async fn publish(app_config: AppConfig, post_content: String, post_status: PostStatus) -> Result<PostResult, post::PostError> {
    let post = Post::new(
        post_content,
        post_status,
        app_config.service);
    post.publish().await
}
