use mp::cli::run;

/// Main entry point for the CLI application.
#[tokio::main]
async fn main() {
    run().await;
}
