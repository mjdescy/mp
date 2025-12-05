# mp: Micropub Publisher

A Rust workspace containing both a command-line tool and library for publishing text-only blog posts to any [Micropub](https://www.w3.org/TR/micropub/)-compatible blogging service.

## Overview

`mp` is a workspace that contains two crates:

1. **[mpcli](mpcli/README.md)**: A command-line tool for publishing text-only blog posts to any Micropub-compatible service
2. **[mplib](mplib/README.md)**: A library for integrating Micropub publishing functionality into Rust applications

Both components support publishing posts and drafts with text-only content from command-line arguments, files, or stdin (CLI), or via a programmatic API (library).

## Crates

### mpcli | Micropub Publisher Command-Line Interface

A simple, fast, and reliable command-line utility for publishing blog posts to any Micropub-compatible service.

#### Quick start

```bash
mp configure
mp post "Hello, world!"
```

#### Features

- ✅ Publish posts directly to Micropub endpoints
- ✅ Read content from multiple sources (arguments, files, stdin)
- ✅ Post titles are optional
- ✅ Post titles can be extracted from line 1 of post content
- ✅ Quiet mode for scripting
- ✅ Configuration management with interactive setup
- ✅ Built with Rust for performance and reliability

For more information, see [mpcli/README.md](mpcli/README.md).

### mplib | Micropub Publisher Library

A Rust library that provides functionality for publishing content to Micropub endpoints.

#### Features

- ✅ Publish posts and drafts to Micropub endpoints
- ✅ Handle Micropub service authentication with API tokens
- ✅ Async/await support with Tokio
- ✅ Built with Rust for performance and reliability

For more information, see [mplib/README.md](mplib/README.md).

## Compatible Microblogging Services

`mp` is known to work with the microblogging services listed below.

| Service Name | API URL |
|---------|------------------|
| [Micro.blog](https://micro.blog/) | https://micro.blog/micropub |

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
