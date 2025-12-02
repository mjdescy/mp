//! Command-line interface module for the mp CLI application.
//!
//! This module handles all CLI argument parsing and command execution.

use clap::{arg, command, value_parser, Command};

pub mod sub_commands;

/// Main entry point for the CLI application.
/// Parses command-line arguments and executes the appropriate command.
pub async fn run() {
    let matches = command!()
        .subcommand(
            Command::new("post")
                .about("Publish a post")
                .args(common_content_args())
        )
        .subcommand(
            Command::new("draft")
                .about("Create a draft")
                .args(common_content_args())
        )
        .subcommand(
            Command::new("configure")
                .about("Create a configuration file containing your authentication token")
        )
        .get_matches();

    // Dispatch to the appropriate subcommand handler
    if let Some(post_matches) = matches.subcommand_matches("post") {
        sub_commands::handle_post_or_draft_subcommand(post_matches).await;
    } else if let Some(draft_matches) = matches.subcommand_matches("draft") {
        sub_commands::handle_post_or_draft_subcommand(draft_matches).await;
    } else if matches.subcommand_matches("configure").is_some() {
        sub_commands::handle_configure_subcommand();
    } else {
        eprintln!("Error:");
        eprintln!("No subcommand was used.\n");
        eprintln!("For more information, try '--help'.");
        std::process::exit(1);
    }
}

/// Creates common arguments shared by post and draft commands.
fn common_content_args() -> Vec<clap::Arg> {
    vec![
        arg!([content] "The content of the post")
            .value_parser(value_parser!(String)),
        arg!(-t --title <TITLE> "The title of the post (optional)")
            .value_parser(value_parser!(String)),
        arg!(-f --file <PATH> "Read content from file at PATH")
            .value_parser(value_parser!(String)),
        arg!(-s --stdin "Read content from stdin")
            .action(clap::ArgAction::SetTrue),
        arg!(-e --"extract-title" "Extract title from content if it starts with a markdown level 1 header")
            .action(clap::ArgAction::SetTrue),
        arg!(-q --quiet "Suppress output")
            .action(clap::ArgAction::SetTrue),
    ]
}
