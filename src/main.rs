use clap::{arg, command, value_parser, Command};
use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    let matches = command!() // requires `cargo` feature
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
        )
        .subcommand(
            Command::new("configure")
                .about("Create a configuration file containing your authentication token")
        )
        .get_matches();

    if let Some(post_matches) = matches.subcommand_matches("post") {
        let content = if post_matches.get_flag("stdin") {
            read_stdin_content()
        } else if let Some(file_path) = post_matches.get_one::<String>("file") {
            // --file with a value: mp post --file path/to/file
            read_file_content(file_path)
        } else if let Some(content_arg) = post_matches.get_one::<String>("content") {
            // Direct content: mp post "content"
            Ok(content_arg.clone())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Either provide content as an argument, use --file with a path, or use --stdin flag"
            ))
        };
        
        match content {
            Ok(text) => {
                println!("Publishing post with content: {}", text);
                // Logic to publish the post
            }
            Err(e) => {
                eprintln!("Error reading content: {}", e);
                std::process::exit(1);
            }
        }
    } else if let Some(draft_matches) = matches.subcommand_matches("draft") {
        let content = if draft_matches.get_flag("stdin") {
            read_stdin_content()
        } else if let Some(file_path) = draft_matches.get_one::<String>("file") {
            // --file with a value: mp draft --file path/to/file
            read_file_content(file_path)
        } else if let Some(content_arg) = draft_matches.get_one::<String>("content") {
            // Direct content: mp draft "content"
            Ok(content_arg.clone())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Either provide content as an argument, use --file with a path, or use --stdin flag"
            ))
        };
        
        match content {
            Ok(text) => {
                println!("Creating draft with content: {}", text);
                // Logic to create a draft
            }
            Err(e) => {
                eprintln!("Error reading content: {}", e);
                std::process::exit(1);
            }
        }
    } else if matches.subcommand_matches("configure").is_some() {
        println!("Configuring authentication token...");
        // Logic to configure authentication token
    } else {
        println!("No valid subcommand was used.");
    }
}

fn read_file_content(file_path: &str) -> Result<String, std::io::Error> {
    if !Path::new(file_path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", file_path)
        ));
    }
    fs::read_to_string(file_path)
}

fn read_stdin_content() -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}