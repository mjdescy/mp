//! Micropub configuration loading.
//!
//! Settings are read from `$XDG_CONFIG_HOME/mptui/config.toml` by default,
//! falling back to `~/.config/mptui/config.toml`.
//! Parsing is kept free of file access so it can be unit tested without touching real files.

use std::{
    fs, io,
    path::{Component, Path, PathBuf},
};

/// Micropub settings loaded from `$XDG_CONFIG_HOME/mptui/config.toml` by default.
#[derive(Default)]
pub(crate) struct MicropubSettings {
    /// Endpoint configuration state, driving the header display.
    pub(crate) endpoint: EndpointState,
    /// Full API URL used to publish; present when api_url is configured and non-empty.
    pub(crate) service_api_url: Option<String>,
    /// Auth token used to publish; present when auth_token is configured and non-empty.
    pub(crate) service_auth_token: Option<String>,
    /// Whether to extract a title from a leading markdown header, per [default_behavior].
    pub(crate) extract_title: bool,
}

/// What the header endpoint display knows about the Micropub configuration.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) enum EndpointState {
    /// Fully configured; carries the API URL with protocol prefix stripped.
    Configured(String),
    /// api_url is set but auth_token is missing or empty; carries the stripped URL.
    MissingToken(String),
    /// No api_url configured.
    #[default]
    Missing,
    /// The config file could not be read; carries the OS error message.
    ReadError(String),
    /// The config file could not be parsed as TOML; carries the parse error.
    ParseError(String),
}

/// Default location of the mp config file: `$XDG_CONFIG_HOME/mptui/config.toml`,
/// falling back to `~/.config/mptui/config.toml`. Returns `None` when no
/// usable base directory exists.
pub(crate) fn config_file_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|dir| dir.is_absolute())
        .or_else(|| std::env::home_dir().map(|home| home.join(".config")))?;
    Some(base.join("mptui/config.toml"))
}

/// Read the Micropub settings from the default config path.
pub(crate) fn load_micropub_settings() -> MicropubSettings {
    config_file_path()
        .map(|path| load_micropub_settings_from(&path))
        .unwrap_or_default()
}

/// Read the Micropub settings from an explicit path. A missing file is
/// normal for first-time users and yields default (unconfigured) settings.
pub(crate) fn load_micropub_settings_from(path: &Path) -> MicropubSettings {
    match fs::read_to_string(path) {
        Ok(config_content) => parse_micropub_settings(&config_content),
        Err(e) if e.kind() == io::ErrorKind::NotFound => MicropubSettings::default(),
        Err(e) => MicropubSettings {
            endpoint: EndpointState::ReadError(e.to_string()),
            ..Default::default()
        },
    }
}

/// Resolve the draft autosave path: `$XDG_STATE_HOME/mptui/draft.md`,
/// falling back to `~/.local/state/mptui/draft.md`. Returns `None` when no
/// usable base directory exists, which disables autosave.
pub(crate) fn default_draft_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::home_dir().map(|home| home.join(".local/state")))?;
    Some(base.join("mptui/draft.md"))
}

/// Parse Micropub settings from the contents of an mp config file.
pub(crate) fn parse_micropub_settings(config_content: &str) -> MicropubSettings {
    let mut settings = MicropubSettings::default();

    let config: toml::Table = match config_content.parse() {
        Ok(config) => config,
        Err(e) => {
            let first_line = e
                .to_string()
                .lines()
                .next()
                .unwrap_or("invalid TOML")
                .to_string();
            settings.endpoint = EndpointState::ParseError(first_line);
            return settings;
        }
    };

    let read_service_value = |key: &str| {
        config
            .get("service")?
            .get(key)?
            .as_str()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
    };
    let api_url = read_service_value("api_url");
    let auth_token = read_service_value("auth_token");

    if let Some(api_url) = api_url {
        let display = strip_url_protocol(&api_url).to_string();
        settings.service_api_url = Some(api_url);
        settings.endpoint = if auth_token.is_some() {
            EndpointState::Configured(display)
        } else {
            EndpointState::MissingToken(display)
        };
    }
    settings.service_auth_token = auth_token;

    settings.extract_title = config
        .get("default_behavior")
        .and_then(|section| section.get("extract_title"))
        .and_then(|value| value.as_bool())
        .unwrap_or(false);

    settings
}

/// Turn a user-supplied path into a fully resolved absolute path: relative
/// paths are joined to the current directory, and symlinks and `.`/`..`
/// segments are resolved. The file itself need not exist (it may be about to
/// be created), so the longest existing ancestor is canonicalized and the
/// remaining components are normalized lexically.
pub(crate) fn resolve_path(path: &Path) -> io::Result<PathBuf> {
    let absolute = std::path::absolute(path)?;
    let Some((existing, canonical)) = absolute
        .ancestors()
        .find_map(|ancestor| Some((ancestor, fs::canonicalize(ancestor).ok()?)))
    else {
        return Ok(absolute);
    };
    let mut resolved = canonical;
    let tail = absolute.strip_prefix(existing).unwrap_or(Path::new(""));
    for component in tail.components() {
        match component {
            Component::Normal(name) => resolved.push(name),
            Component::ParentDir => _ = resolved.pop(),
            _ => {}
        }
    }
    Ok(resolved)
}

/// Remove the leading `https://` or `http://` protocol prefix from a URL.
pub(crate) fn strip_url_protocol(url: &str) -> &str {
    url.strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_settings_reports_configured_endpoint() {
        let settings = parse_micropub_settings(
            "[service]\napi_url = \"https://example.com/micropub\"\nauth_token = \"secret\"\n",
        );

        assert_eq!(
            settings.endpoint,
            EndpointState::Configured("example.com/micropub".to_string())
        );
        assert_eq!(
            settings.service_api_url.as_deref(),
            Some("https://example.com/micropub")
        );
        assert_eq!(settings.service_auth_token.as_deref(), Some("secret"));
    }

    #[test]
    fn parse_settings_reports_missing_token() {
        let settings =
            parse_micropub_settings("[service]\napi_url = \"https://example.com/micropub\"\n");

        assert_eq!(
            settings.endpoint,
            EndpointState::MissingToken("example.com/micropub".to_string())
        );
        assert_eq!(
            settings.service_api_url.as_deref(),
            Some("https://example.com/micropub")
        );
        assert_eq!(settings.service_auth_token, None);
    }

    #[test]
    fn parse_settings_reports_missing_endpoint() {
        let settings = parse_micropub_settings("[service]\nauth_token = \"secret\"\n");
        assert_eq!(settings.endpoint, EndpointState::Missing);

        let settings = parse_micropub_settings("");
        assert_eq!(settings.endpoint, EndpointState::Missing);

        let settings = parse_micropub_settings("[service]\napi_url = \"   \"\n");
        assert_eq!(settings.endpoint, EndpointState::Missing);
    }

    #[test]
    fn parse_settings_reports_invalid_toml() {
        let settings = parse_micropub_settings("[service\napi_url = ");
        assert!(matches!(settings.endpoint, EndpointState::ParseError(_)));
    }

    #[test]
    fn parse_settings_reads_title_extraction() {
        let settings = parse_micropub_settings("[default_behavior]\nextract_title = true\n");
        assert!(settings.extract_title);

        let settings = parse_micropub_settings("");
        assert!(!settings.extract_title);
    }

    #[test]
    fn resolve_path_normalizes_dots_for_missing_files() {
        let base = fs::canonicalize(std::env::temp_dir()).unwrap();
        let dir = base.join(format!("mptui-resolve-{}", std::process::id()));
        fs::create_dir_all(dir.join("real")).unwrap();

        let messy = dir.join("real/../real/./missing/../new/config.toml");
        assert_eq!(
            resolve_path(&messy).unwrap(),
            dir.join("real/new/config.toml")
        );

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(dir.join("real"), dir.join("link")).unwrap();
            assert_eq!(
                resolve_path(&dir.join("link/config.toml")).unwrap(),
                dir.join("real/config.toml")
            );
        }
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn loading_missing_file_yields_default_settings() {
        let settings =
            load_micropub_settings_from(std::path::Path::new("/nonexistent-dir/config.toml"));
        assert_eq!(settings.endpoint, EndpointState::Missing);
    }
}
