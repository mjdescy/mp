//! Handle the 'configure' subcommand for the CLI application.

use mplib::MicropubService;
use rustyline::DefaultEditor;
use std::io;

use crate::configuration::app_config::AppConfig;
use crate::configuration::default_behavior::DefaultBehavior;

/// Handle the 'configure' subcommand.
pub fn handle_configure_subcommand() {
    if let Err(e) = configure_app() {
        eprintln!("Error during configuration:");
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn configure_app() -> io::Result<()> {
    if AppConfig::config_file_exists() && !user_opts_to_update_existing_config() {
        println!("Configuration not updated. Exiting.");
        return Ok(());
    }

    let app_config = AppConfig::from_user_input()?;
    let app_config_path = app_config.save()?;

    println!(
        "Configuration saved successfully in file '{}'.",
        app_config_path
    );

    Ok(())
}

fn user_opts_to_update_existing_config() -> bool {
    let config_file_path =
        AppConfig::get_config_file_path().unwrap_or_else(|_| String::from("unknown path"));

    println!(
        "Configuration file already exists in file '{}'.",
        config_file_path
    );
    let response = get_user_input("Do you want to update it (y/N)? ");

    response.trim().eq_ignore_ascii_case("y")
}

trait UserInputConfigurable: Sized {
    fn from_user_input() -> io::Result<Self>;
}

impl UserInputConfigurable for AppConfig {
    fn from_user_input() -> io::Result<Self> {
        let microblog_service = MicropubService::from_user_input()?;
        let default_behavior = DefaultBehavior::from_user_input()?;

        let app_config = AppConfig::new(microblog_service, default_behavior);
        Ok(app_config)
    }
}

impl UserInputConfigurable for MicropubService {
    fn from_user_input() -> io::Result<Self> {
        println!();
        println!("==============================================");
        println!("Define the micopub API endpoint to publish to.");
        println!("==============================================");

        let api_url = get_user_input("[Step 1 of 2] Enter API URL");
        let auth_token = get_user_input("[Step 2 of 2] Enter Authentication Token");

        Self::from_args(api_url, auth_token)
    }
}

impl UserInputConfigurable for DefaultBehavior {
    fn from_user_input() -> io::Result<Self> {
        println!();
        println!("=======================================================");
        println!("Define default behavior for the application.");
        println!("These options can be overridden via command-line flags.");
        println!("=======================================================");

        let quiet =
            get_user_input("[Step 1 of 2] Enable quiet mode, which suppresses output (y/N)");
        let extract_title =
            get_user_input("[Step 2 of 2] Extract titles from line 1 content (Y/n)");

        Self::from_args(quiet, extract_title)
    }
}

fn get_user_input(prompt: &str) -> String {
    let prompt = format!("{}: ", prompt);
    let mut rl = DefaultEditor::new().expect("Failed to create readline editor");
    let readline = rl.readline(&prompt);

    match readline {
        Ok(line) => line.trim().to_string(),
        Err(_) => String::new(),
    }
}
