# mplib: Micropub Publisher Library

A Rust library for publishing text-only blog posts to any [Micropub](https://www.w3.org/TR/micropub/)-compatible blogging service.

## Overview

`mplib` can be integrated into your own applications to add Micropub publishing capabilities of text-only content.

## Features

- ✅ Publish posts and drafts to Micropub endpoints
- ✅ Handle Micropub service authentication with API tokens
- ✅ Async/await support with Tokio
- ✅ Built with Rust for performance and reliability

## Compatible Microblogging Services

`mplib` is known to work with the microblogging services listed below.

| Service Name | API URL |
|---------|------------------|
| [Micro.blog](https://micro.blog/) | https://micro.blog/micropub |

## Installation

Add `mplib` to your `Cargo.toml`:

```toml
[dependencies]
mplib = { version = "0.1.0" }  # Use appropriate version
tokio = { version = "1.0", features = ["full"] }
```

## Usage

```rust
use mplib::{MicropubService, Post, PostStatus, publish_post};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Post
    let post = Post::from_body_and_title(
        "This is the post body".to_string(),
        "Post title".to_string(),
        PostStatus::Published  // change to PostStatus::Draft to create a draft
    )
    
    // Create a MicropubService configuration
    let service = MicropubService::new(
        "https://micro.blog/micropub".to_string(),
        "your-auth-token".to_string()
    );

    // Publish the Post on the MicropubService
    match publish_post(post, service).await {
        Ok(result) => {
            println!("{}", result.as_string());
        }
        Err(e) => {
            eprintln!("Error publishing post");
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
```

## Requirements

- Rust 1.70+
- A Micropub-compatible blogging service

## License

This project is licensed under the MIT License - see the [LICENSE.md](../LICENSE.md) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

Michael Descy <mike@mjdescy.me>

## Support

If you encounter any issues or have questions, please file an issue on the GitHub repository.
