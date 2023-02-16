use std::path::PathBuf;

const BASE: &'static str = ".phosphorus";
const DATA: &'static str = "data";
const CACHE: &'static str = "cache";
const DOWNLOAD: &'static str = "download";

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
}

impl Paths {
    pub fn new(base: PathBuf, data: PathBuf, cache: PathBuf, download: PathBuf) -> Self {
        Paths {
            base,
            data,
            cache,
            download,
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

    let cache = data.join(CACHE);
    check_folder(&cache, CACHE)?;
    let download = data.join(DOWNLOAD);
    check_folder(&download, DOWNLOAD)?;

    Ok(Paths::new(base, data, cache, download))
}

fn check_folder(path: &std::path::PathBuf, dir: &'static str) -> Result<(), ConfigError> {
    match std::fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(msg) => {
            return Err(ConfigError::DirCreationError(dir, msg.to_string()));
        }
    }
}
