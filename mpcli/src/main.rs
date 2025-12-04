mod cli;
mod configuration;

use cli::run;
use tokio;

/// Main entry point for the CLI application.
#[tokio::main]
async fn main() {
    run().await;
}
