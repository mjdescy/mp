# mp: Micropub Publisher

A command-line tool to publish text-only blog posts to Micropub endpoints.

## Overview

`mp` is a simple, fast, and reliable command-line utility for publishing blog posts to any Micropub-compatible service. It supports publishing posts and drafts with content from command-line arguments, files, or stdin.

**`mp` can also be used as a Rust library** for integrating Micropub publishing functionality into your own applications.

## Features

- ✅ Publish posts directly to Micropub endpoints
- ✅ Create drafts
- ✅ Read content from multiple sources (arguments, files, stdin)
- ✅ Configuration management with interactive setup
- ✅ Quiet mode for scripting
- ✅ Built with Rust for performance and reliability
- ✅ **Library API for use in other Rust projects**

## Compatible Microblogging Services

`mp` is known to work with the microblogging services listed below.

| Service Name | API URL |
|---------|------------------|
| [Micro.blog](https://micro.blog/) | https://micro.blog/micropub |

## Installation

### From Source

```bash
git clone https://github.com/mjdescy/mp.git
cd mp
cargo build --release
```

The binary will be available at `target/release/mp`.

### As a Library

Add `mp` to your `Cargo.toml`:

```toml
[dependencies]
mp = { path = "../mp" }  # Use appropriate path or version
tokio = { version = "1.0", features = ["full"] }
```

## Configuration

Before using `mp`, you need to configure your Micropub service settings:

```bash
mp configure
```

This will prompt you to enter:
1. **API URL**: Your Micropub API endpoint URL (refer to the "Compatible Microblogging Services" table above)
2. **Authentication Token**: Your Micropub authentication token

The configuration is saved locally and will be used for all subsequent posts.

## Library Usage

If you want to use `mp` as a library in your Rust application:

```rust
use mp::{publish, publish_post, publish_draft, AppConfig, MicroblogService, PostStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a MicroblogService configuration
    let service = MicroblogService::new(
        "https://micro.blog/micropub".to_string(),
        "your-auth-token".to_string()
    );
    let app_config = AppConfig::new(service);
    
    // Publish a post
    let result = publish_post(app_config, "Hello from the library!".to_string(), false).await?;
    println!("Published at: {}", result.url);
    
    // Or use the more general publish function
    let result = publish(
        app_config, 
        "Draft content".to_string(), 
        PostStatus::Draft, 
        false
    ).await?;
    
    println!("Draft created at: {}", result.url);
    
    Ok(())
}
```

### Loading Configuration from File

You can also load configuration from the standard config file:

```rust
use mp::{publish_post, AppConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load existing configuration (same as CLI tool uses)
    let app_config = AppConfig::load()?;
    
    let result = publish_post(app_config, "Post content".to_string(), true).await?;
    
    Ok(())
}
```

## Usage

### Publishing Posts

Publish a post with content as an argument:
```bash
mp post "Hello, world! This is my first post."
```

Publish a post from a file:
```bash
mp post --file my-post.txt
```

Publish a post from stdin:
```bash
echo "Hello from stdin!" | mp post --stdin
```

### Creating Drafts

Create a draft (same syntax as posts):
```bash
mp draft "This is a draft post."
mp draft --file draft.txt
mp draft --stdin
```

### Quiet Mode

Use the `--quiet` or `-q` flag to suppress output (useful for scripting):
```bash
mp post --quiet "Silent post"
```

## Command Reference

### `mp post [OPTIONS] [CONTENT]`

Publish a post to your configured Micropub endpoint.

**Options:**
- `[CONTENT]` - The content of the post as a command-line argument
- `-f, --file <PATH>` - Read content from file at PATH
- `-s, --stdin` - Read content from stdin
- `-q, --quiet` - Suppress output

### `mp draft [OPTIONS] [CONTENT]`

Create a draft post (same options as `post`).

### `mp configure`

Create or update the configuration file containing your authentication settings.

## Configuration File

The configuration is stored in a TOML file at the standard configuration directory for your platform. On Unix-like systems, this path is `~/.config/mp/config.toml`.

The file contains:

```toml
[microblog_service]
api_url = "https://your-micropub-endpoint.example.com"
auth_token = "your-authentication-token"
```

## Examples

```bash
# Configure the tool (first time setup)
mp configure

# Publish a simple post
mp post "Just finished reading a great book!"

# Publish a longer post from a file
mp post --file blog-post.md

# Create a draft for later
mp draft "Working on this idea..."

# Silent posting for automation
echo "Automated post" | mp post --stdin --quiet
```

## Requirements

- Rust 1.70+ (for building from source)
- A Micropub-compatible blogging service

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

Michael Descy <mike@mjdescy.me>

## Support

If you encounter any issues or have questions, please file an issue on the GitHub repository.
