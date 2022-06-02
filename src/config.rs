use std::path::PathBuf;

use color_eyre::{eyre::Context, Result};
use figment::{providers::Env, Figment};
use once_cell::sync::Lazy;
use serde::Deserialize;

/// Change this to your own prefix.
const CONFIG_PREFIX: &str = "APP_";

/// Default values for config, used with #[serde(default = "..")]
mod default {
    use std::path::PathBuf;

    pub fn some_path() -> PathBuf {
        PathBuf::from("/some/path")
    }
}

/// Config of the application, loaded from environment variables.
/// If load from other files (yaml, toml, etc ..) is desired, add more features
/// of [`figment`] and modify [`Config::from_env`],  potentially renaing
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // Add your config fields here
    #[serde(default = "default::some_path")]
    pub some_path: PathBuf,
}

impl Config {
    /// Loads the config once and cache it. Returns a
    /// reference to the cache.
    pub fn get<'a>() -> &'a Self {
        static CONF: Lazy<Config> = Lazy::new(|| Config::from_env().unwrap());
        &CONF
    }

    /// Extract the config from environment variables.
    pub fn from_env() -> Result<Self> {
        Figment::new()
            .merge(Env::prefixed(CONFIG_PREFIX))
            .extract()
            .wrap_err("Failed to load config from env")
    }
}

#[test]
fn test_config() {
    figment::Jail::expect_with(|jail| {
        jail.set_env(format!("{}SOME_PATH", CONFIG_PREFIX), "/some/path");
        let conf = Config::from_env().unwrap();
        assert_eq!(conf.some_path, PathBuf::from("/some/path"));
        Ok(())
    })
}
