pub mod post;
pub mod post_result;
pub mod post_status;

use crate::publish::post::Post;
pub use crate::publish::post_result::PostResult;
pub use crate::publish::post_status::PostStatus;
pub use crate::publish::post::PostError;
use crate::configure::AppConfig;

pub async fn handle_draft_verb(app_config: AppConfig, post_content: String, quiet: bool) -> Result<PostResult, post::PostError> {
    let post_status = PostStatus::Draft;

    if !quiet {
        println!("Publishing draft with content:");
        println!("{}\n", post_content);
    }

    publish(app_config, post_content, post_status, quiet).await
}

pub async fn handle_post_verb(app_config: AppConfig, post_content: String, quiet: bool) -> Result<PostResult, post::PostError> {
    let post_status = PostStatus::Published;

    if !quiet {
        println!("Publishing post with content:");
        println!("{}\n", post_content);
    }

    publish(app_config, post_content, post_status, quiet).await
}

async fn publish(app_config: AppConfig, post_content: String, post_status: PostStatus, quiet: bool) -> Result<PostResult, post::PostError> {
    let post = Post::new(
        post_content,
        post_status,
        app_config.service);
    let post_result = post.publish().await;
    match post_result {
        Ok(result) => {
            if !quiet {
                println!("{}", result.as_string());
            }
            Ok(result)
        },
        Err(e) => {
            Err(e)
        }
    }
}
