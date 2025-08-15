use clap;
use std::fs;
use std::io;
use std::io::Read;

pub fn get_content_from_args(matches: &clap::ArgMatches) -> Result<String, std::io::Error> {
    let content = if matches.get_flag("stdin") {
        read_stdin_content()
    } else if let Some(file_path) = matches.get_one::<String>("file") {
        read_file_content(file_path)
    } else if let Some(content_arg) = matches.get_one::<String>("content") {
        Ok(content_arg.clone())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Either provide content as an argument, use --file with a path, or use --stdin flag"
        ))
    };

    if let Some(text) = content.ok() {
        if text.trim().is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Content cannot be empty"
            ));
        }
        Ok(text)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Failed to read content"
        ))
    }
}

fn read_file_content(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

fn read_stdin_content() -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}