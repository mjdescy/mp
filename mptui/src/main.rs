mod app;
mod config;
mod publish;
mod setup;

use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::Parser;

/// Terminal UI for writing and publishing Micropub posts.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Use this configuration file instead of the default
    #[arg(
        short,
        long,
        value_name = "path",
        value_parser = resolved_path,
        overrides_with = "config"
    )]
    config: Option<PathBuf>,
}

/// Reject empty paths and fully resolve the rest (absolute, symlinks and
/// `..` resolved) so the app only ever sees one canonical form of the path.
/// The file does not need to exist yet.
fn resolved_path(value: &str) -> Result<PathBuf, String> {
    if value.is_empty() {
        return Err("the configuration file path must not be empty".to_string());
    }
    config::resolve_path(Path::new(value)).map_err(|e| format!("cannot resolve path: {e}"))
}

fn main() -> ExitCode {
    let config_override = Cli::parse().config;

    let mut terminal = ratatui::init();
    let result = run(&mut terminal, config_override);
    ratatui::restore();
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("mptui-ui: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Show the first-run setup wizard when no config file exists, then load
/// the main UI. The app reads the config file when it is built, so a file
/// written by the wizard is picked up before the main UI appears. A path
/// given with `--config` replaces the default location everywhere.
fn run(
    terminal: &mut ratatui::DefaultTerminal,
    config_override: Option<PathBuf>,
) -> std::io::Result<()> {
    let Some(path) = config_override.or_else(config::config_file_path) else {
        return app::App::default().run(terminal);
    };
    if !path.exists()
        && setup::run(terminal, &path, setup::SetupMode::FirstRun)? == setup::SetupOutcome::Exited
    {
        return Ok(());
    }
    app::App::with_config_path(path).run(terminal)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Option<PathBuf>, clap::Error> {
        Cli::try_parse_from(std::iter::once("mptui-ui").chain(args.iter().copied()))
            .map(|cli| cli.config)
    }

    /// The expected result for `value`: relative paths are joined to the
    /// (canonical) current directory.
    fn path(value: &str) -> Option<PathBuf> {
        Some(
            std::fs::canonicalize(std::env::current_dir().unwrap())
                .unwrap()
                .join(value),
        )
    }

    #[test]
    fn no_arguments_uses_default_config() {
        assert_eq!(parse(&[]).unwrap(), None);
    }

    #[test]
    fn config_path_accepted_in_all_forms() {
        assert_eq!(parse(&["-c", "a.toml"]).unwrap(), path("a.toml"));
        assert_eq!(parse(&["--config", "a.toml"]).unwrap(), path("a.toml"));
        assert_eq!(parse(&["--config=a.toml"]).unwrap(), path("a.toml"));
    }

    #[test]
    fn relative_config_path_is_resolved() {
        let parsed = parse(&["-c", "sub/a.toml"]).unwrap().unwrap();
        assert!(parsed.is_absolute());
        assert!(parsed.ends_with("sub/a.toml"));

        let dotted = parse(&["-c", "sub/../a.toml"]).unwrap().unwrap();
        assert_eq!(Some(dotted), path("a.toml"));
    }

    #[test]
    fn absolute_config_path_is_kept() {
        assert_eq!(
            parse(&["-c", "/etc/mp/config.toml"]).unwrap(),
            Some(PathBuf::from("/etc/mp/config.toml"))
        );
    }

    #[test]
    fn last_config_path_wins() {
        assert_eq!(
            parse(&["-c", "a.toml", "-c", "b.toml"]).unwrap(),
            path("b.toml")
        );
    }

    #[test]
    fn bad_arguments_are_rejected() {
        assert!(parse(&["-c"]).is_err());
        assert!(parse(&["--config="]).is_err());
        assert!(parse(&["--bogus"]).is_err());
    }
}
