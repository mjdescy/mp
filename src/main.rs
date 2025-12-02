use mp::cli;

/// Main entry point for the CLI application.
#[tokio::main]
async fn main() {
    cli::run().await;
}
