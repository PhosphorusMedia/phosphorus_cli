use std::path::PathBuf;

const BASE: &'static str = ".phosphorus";
const DATA: &'static str = "songs_meta";
const CACHE: &'static str = "cache";
const DOWNLOAD: &'static str = "download";
const PLAYLISTS: &'static str = "playlists_meta";

#[derive(Debug)]
pub enum ConfigError {
    NoHomeDir,
    DirCreationError(&'static str, String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::NoHomeDir => write!(f, "No home dir could be found for this user"),
            ConfigError::DirCreationError(dir, msg) => write!(
                f,
                "Directory `{}` couldn't be created. The following error was thrown: {}",
                dir, msg
            ),
        }
    }
}

impl std::error::Error for ConfigError {}

pub struct Paths {
    base: PathBuf,
    data: PathBuf,
    cache: PathBuf,
    download: PathBuf,
    playlists: PathBuf,
}

impl Paths {
    pub fn new(
        base: PathBuf,
        data: PathBuf,
        cache: PathBuf,
        download: PathBuf,
        playlists: PathBuf,
    ) -> Self {
        Paths {
            base,
            data,
            cache,
            download,
            playlists,
        }
    }

    pub fn base(&self) -> &PathBuf {
        &self.base
    }

    pub fn base_as_str(&self) -> &str {
        &self.base.to_str().unwrap()
    }

    pub fn data(&self) -> &PathBuf {
        &self.data
    }

    pub fn data_as_str(&self) -> &str {
        &self.data.to_str().unwrap()
    }

    pub fn cache(&self) -> &PathBuf {
        &self.cache
    }

    pub fn cache_as_str(&self) -> &str {
        &self.cache.to_str().unwrap()
    }

    pub fn download(&self) -> &PathBuf {
        &self.download
    }

    pub fn download_as_str(&self) -> &str {
        &self.download.to_str().unwrap()
    }

    pub fn playlists(&self) -> &PathBuf {
        &self.playlists
    }

    pub fn playlists_as_str(&self) -> &str {
        &self.playlists.to_str().unwrap()
    }
}

/// Configures the environment creating the necessary folders.
/// Returns a `Paths` instance holding paths for all the created
/// folders.
pub fn config_env() -> Result<Paths, ConfigError> {
    let user_dirs = match directories::UserDirs::new() {
        Some(dirs) => dirs,
        None => {
            return Err(ConfigError::NoHomeDir);
        }
    };
    let home = user_dirs.home_dir();

    let base = home.join(BASE);
    check_folder(&base, BASE)?;

    let data = base.join(DATA);
    check_folder(&data, DATA)?;

    let cache = base.join(CACHE);
    check_folder(&cache, CACHE)?;

    let download = base.join(DOWNLOAD);
    check_folder(&download, DOWNLOAD)?;

    let playlists = base.join(PLAYLISTS);
    check_folder(&playlists, PLAYLISTS)?;

    Ok(Paths::new(base, data, cache, download, playlists))
}

fn check_folder(path: &std::path::PathBuf, dir: &'static str) -> Result<(), ConfigError> {
    match std::fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(msg) => {
            return Err(ConfigError::DirCreationError(dir, msg.to_string()));
        }
    }
}
