use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::path::PathBuf;

#[derive(Debug, Snafu)]
/// Devenv error types.
pub enum Error {
    #[snafu(display(
        "Missing .dev project folder in the current working directory or his parents."
    ))]
    ProjectNotFound,
    #[snafu(display("Could not open config from {}: {}", path.display(), source))]
    OpenConfig {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Invalid config file {}: {}", path.display(), source))]
    InvalidConfig {
        path: PathBuf,
        source: toml::de::Error,
    },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    plugins: Vec<String>,
}

/// Search for a directory containing the .dev folder.
/// Begins looking from the current working director and iterate upward.
/// Returns `None` if no such directoring is found, otherwise returns the
/// directory containing folder .dev folder.
pub fn find_project_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir().map_err(|_| ProjectNotFound {}.build())?;

    cwd.ancestors()
        .map(|path| path.join(".dev"))
        .find(|path| path.exists())
        .ok_or_else(|| ProjectNotFound {}.build())
        .map(|path| path.into())
}

impl Config {
    pub fn load(path: Option<PathBuf>) -> Result<Config> {
        match path {
            Some(path) => Config::from_path(path),
            None => Config::from_path(find_project_root()?.join("config.toml")),
        }
    }

    pub fn from_path(path: PathBuf) -> Result<Config> {
        let config_file =
            std::fs::read_to_string(&path).with_context(|| OpenConfig { path: path.clone() })?;
        toml::from_str(&config_file).context(InvalidConfig { path })
    }
}
