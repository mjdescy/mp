mod configure;
mod get_content;
mod publish;

use clap::{arg, command, value_parser, Command};

use crate::configure::handle_configure_verb;
use crate::publish::{handle_draft_verb, handle_post_verb};
use crate::configure::AppConfig;
use crate::get_content::get_content_from_args;

#[tokio::main]
async fn main() {
    let matches = command!()
        .subcommand(
            Command::new("post")
                .about("Publish a post")
                .arg(
                    arg!([content] "The content of the post")
                        .value_parser(value_parser!(String))
                )
                .arg(
                    arg!(-f --file <PATH> "Read content from file at PATH")
                        .value_parser(value_parser!(String))
                )
                .arg(
                    arg!(-s --stdin "Read content from stdin")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    arg!(-q --quiet "Suppress output")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("draft")
                .about("Create a draft")
                .arg(
                    arg!([content] "The content of the draft")
                        .value_parser(value_parser!(String))
                )
                .arg(
                    arg!(-f --file <PATH> "Read content from file at PATH")
                        .value_parser(value_parser!(String))
                )
                .arg(
                    arg!(-s --stdin "Read content from stdin")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    arg!(-q --quiet "Suppress output")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("configure")
                .about("Create a configuration file containing your authentication token")
        )
        .get_matches();

    if let Some(post_matches) = matches.subcommand_matches("post") {
        let content = get_content_from_args(post_matches);
        let quiet = post_matches.get_flag("quiet");
        
        match content {
            Ok(text) => {
                let app_config = AppConfig::load().unwrap_or_else(|e| {
                    eprintln!("Error loading configuration: {}", e);
                    std::process::exit(1);
                });

                handle_post_verb(app_config, text, quiet)
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("Error publishing post: {}", e);
                        std::process::exit(1);
                    }); 
            }
            Err(e) => {
                eprintln!("Error reading content: {}", e);
                std::process::exit(1);
            }
        }
    } else if let Some(draft_matches) = matches.subcommand_matches("draft") {
        let content = get_content_from_args(draft_matches);
        let quiet = draft_matches.get_flag("quiet");

        match content {
            Ok(text) => {
                let app_config = AppConfig::load().unwrap_or_else(|e| {
                    eprintln!("Error loading configuration: {}", e);
                    std::process::exit(1);
                });

                handle_draft_verb(app_config, text, quiet)
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("Error publishing post: {}", e);
                        std::process::exit(1);
                    }); 
            }
            Err(e) => {
                eprintln!("Error reading content: {}", e);
                std::process::exit(1);
            }
        }
    } else if matches.subcommand_matches("configure").is_some() {
        match handle_configure_verb() {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error during configuration: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        println!("No valid subcommand was used.");
    }
}
