# mpcli: Micropub Publisher Command Line Interface

A command-line tool for publishing text-only blog posts to any [Micropub](https://www.w3.org/TR/micropub/)-compatible blogging service.

## Overview

`mp` is a simple, fast, and reliable command-line utility for publishing blog posts to a Micropub-compatible blog. It supports publishing posts and drafts with content (including an optional title) provided by command-line arguments, a text file, or stdin.

## Features

- ✅ Publish posts directly to Micropub endpoints
- ✅ Read content from multiple sources (arguments, files, stdin)
- ✅ Post titles are optional
- ✅ Post titles can be extracted from line 1 of post content
- ✅ Quiet mode for scripting
- ✅ Configuration management with interactive setup
- ✅ Built with Rust for performance and reliability

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

## Configuration

Before using `mp`, you need to configure your Micropub service settings:

```bash
mp configure
```

This will prompt you to enter:
1. **API URL**: Your Micropub API endpoint URL (refer to the "Compatible Microblogging Services" table above)
2. **Authentication Token**: Your Micropub authentication token

The configuration is saved locally and will be used for all subsequent posts.

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

#### Arguments

- `[CONTENT]` - The content of the post as a command-line argument

#### Options

- `-t, --title`:  The title of the post (optional)
- `-f, --file <PATH>` - Read content from file at PATH
- `-s, --stdin` - Read content from stdin
- `-e, --extract-title` - Extract a post title from the first line of post content, if the first line starts with a Markdown level 1 heading ("# ")
- `-q, --quiet` - Suppress output

### `mp draft [OPTIONS] [CONTENT]`

Create a draft post (same options as `post`).

### `mp configure`

Create or update the configuration file containing your authentication settings. `mp configure` will launch a short, guided process that prompts for user input and then outputs a configuration file.

## Configuration File

The configuration is stored in a TOML file at the standard configuration directory for your platform. On Unix-like systems, this path is `~/.config/mp/config.toml`.

To create a config file, simply run `mp configure` and follow the on-screen prompts.

### Example Configuration File

```toml
[microblog_service]
api_url = "https://your-micropub-endpoint.example.com"
auth_token = "your-authentication-token"

[default_behavior]
quiet = false
extract_title = true
```

## Usage Examples

```bash
# Configure the tool (first time setup)
mp configure

# Publish a simple post
mp post "Just finished reading a great book!"

# Publish a simple post with a title
mp post "Just finished reading a great book!" --title "Post title"

# Publish a longer post from a file
mp post --file blog-post.md

# Publish a longer post from a file, 
# and submit the first line in the file as
# the post title if it is a markdown level 1
#  heading ("# ") 
mp post --file blog-post.md --extract-title

# Create a draft for later
mp draft "Working on this idea..."

# Silent posting for automation
echo "Automated post" | mp post --stdin --quiet

# Get command usage information
mp help
mp help post
mp help draft
```

## Requirements

- Rust 1.70+ (for building from source)
- A Micropub-compatible blogging service

## License

This project is licensed under the MIT License - see the [LICENSE.md](../LICENSE.md) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

Michael Descy <mike@mjdescy.me>

## Support

If you encounter any issues or have questions, please file an issue on the GitHub repository.
