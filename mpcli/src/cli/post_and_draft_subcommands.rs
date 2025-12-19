//! Handle the 'post' and 'draft' subcommands for the CLI application.

use clap::ArgMatches;
use mplib::{Post, PostStatus, publish_post};
use std::{fs, io, io::Read};

use crate::configuration::app_config::AppConfig;

/// Handle the 'post' or 'draft' subcommand.
pub async fn handle_post_or_draft_subcommand(matches: &ArgMatches) {
    let app_config = AppConfig::load().unwrap_or_else(|e| {
        eprintln!("Error loading configuration:");
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let status = if matches.subcommand_name() == Some("draft") {
        PostStatus::Draft
    } else {
        PostStatus::Published
    };

    let post_or_draft = match status {
        PostStatus::Published => "post",
        PostStatus::Draft => "draft",
    };

    let extract_title =
        app_config.default_behavior.extract_title || matches.get_flag("extract-title");

    let post = post_from_args(matches, status, extract_title).unwrap();

    if post.is_empty() {
        eprintln!("Error: {} content cannot be empty", post_or_draft);
        std::process::exit(1);
    }

    let quiet = app_config.default_behavior.quiet || matches.get_flag("quiet");

    match publish_post(post, &app_config.service).await {
        Ok(result) => {
            if quiet {
                return;
            }
            println!("{}", result.as_string());
        }
        Err(e) => {
            eprintln!("Error publishing {}:", post_or_draft);
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

/// Construct a Post from CLI arguments.
fn post_from_args(
    matches: &ArgMatches,
    status: PostStatus,
    extract_title_default: bool,
) -> Result<Post, String> {
    let body = get_post_body_from_cli_args(matches)
        .map_err(|e| format!("Error reading content: {}", e))?;
    let title = matches.get_one::<String>("title").cloned();
    let extract_title = title.is_none() && extract_title_default;

    if extract_title {
        Ok(Post::from_body_with_title_extraction(body, status))
    } else if title.is_some() {
        Ok(Post::from_body_and_title(body, title.unwrap(), status))
    } else {
        Ok(Post::from_body(body, status))
    }
}

/// Retrieve the post body from CLI arguments.
fn get_post_body_from_cli_args(matches: &ArgMatches) -> Result<String, std::io::Error> {
    if matches.get_flag("stdin") {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else if let Some(file_path) = matches.get_one::<String>("file") {
        fs::read_to_string(file_path)
    } else if let Some(content_arg) = matches.get_one::<String>("content") {
        Ok(content_arg.clone())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Either provide content as an argument, use --file with a path, or use --stdin.",
        ))
    }
}
