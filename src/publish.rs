mod post;
mod post_result;
mod post_status;

use crate::publish::post::Post;
use crate::publish::post_result::PostResult;
pub use crate::publish::post_status::PostStatus;
use crate::configure::AppConfig;

pub async fn handle_draft_verb(app_config: AppConfig, post_content: String) -> Result<PostResult, post::PostError> {
    let post_status = PostStatus::Draft;

    println!("Publishing draft with content:");
    println!("{}\n", post_content);

    publish(app_config, post_content, post_status).await
}

pub async fn handle_post_verb(app_config: AppConfig, post_content: String) -> Result<PostResult, post::PostError> {
    let post_status = PostStatus::Published;

    println!("Publishing post with content:");
    println!("{}\n", post_content);

    publish(app_config, post_content, post_status).await
}

async fn publish(app_config: AppConfig, post_content: String, post_status: PostStatus) -> Result<PostResult, post::PostError> {
    let post = Post::new(
        post_content,
        post_status,
        app_config.service);
    let post_result = post.publish().await;
    match post_result {
        Ok(result) => {
            println!("{}", result.as_string());
            Ok(result)
        },
        Err(e) => {
            eprintln!("Error publishing post: {}", e);
            Err(e)
        }
    }
}
