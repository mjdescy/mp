pub(crate) mod microblog_service;
mod app_config;
mod user_input;

use std::io;
use crate::configure::microblog_service::MicroblogService;
pub use crate::configure::app_config::AppConfig;
use crate::configure::user_input::get_user_input;

pub fn handle_configure_verb() -> io::Result<()> {
    if AppConfig::config_file_exists() {
        if !user_opts_to_update_existing_config() {
            println!("Configuration not updated. Exiting.");
            return Ok(());
        }
    }

    let config = MicroblogService::from_user_input()?;
    let app_config = AppConfig::new(config);

    let app_config_path = app_config.save()?;
    println!("Configuration saved successfully in file '{}'.", app_config_path);

    Ok(())
}

fn user_opts_to_update_existing_config() -> bool {
    let config_file_path = AppConfig::get_config_file_path()
        .unwrap_or_else(|_| String::from("unknown path"));

    println!("Configuration file already exists in file '{}'.", config_file_path);
    let response = get_user_input("Do you want to update it (y/N)? ");

    response.trim().eq_ignore_ascii_case("y")
}

#[allow(dead_code)]
pub fn load_config() -> Result<AppConfig, io::Error> {
    if !AppConfig::config_file_exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Configuration file not found. Run mp configure to create a configuraiton file."));
    }

    AppConfig::load()
}
