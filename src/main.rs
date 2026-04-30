mod config;
mod hyprland;
mod sensor;

use anyhow::{Context, Result};
use futures_util::stream::StreamExt;
use log::{error, info, warn};
use std::env;
use std::path::{PathBuf, Path};
use std::time::SystemTime;
use crate::config::LuaConfig;

struct ConfigCache {
    path: PathBuf,
    config: Option<LuaConfig>,
    last_mtime: Option<SystemTime>,
}

impl ConfigCache {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            config: None,
            last_mtime: None,
        }
    }

    fn get_config(&mut self) -> Result<&LuaConfig> {
        let mtime = std::fs::metadata(&self.path)
            .and_then(|m| m.modified())
            .ok();

        if self.config.is_none() || mtime != self.last_mtime {
            match LuaConfig::new(&self.path) {
                Ok(new_config) => {
                    info!("Configuration (re)loaded: {:?}", self.path);
                    self.config = Some(new_config);
                    self.last_mtime = mtime;
                }
                Err(e) => {
                    error!("Failed to load config: {}. Keeping previous version.", e);
                    if self.config.is_none() {
                        return Err(e);
                    }
                }
            }
        }
        Ok(self.config.as_ref().unwrap())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config_path = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = env::var("HOME").expect("HOME environment variable is not set");
            PathBuf::from(home).join(".config")
        })
        .join("hypr/hyprspin.lua");

    let mut cache = ConfigCache::new(config_path);
    
    info!("hyprspin: daemon started");

    let proxy = sensor::get_proxy().await
        .context("Failed to connect to iio-sensor-proxy")?;

    if let Ok(initial) = proxy.accelerometer_orientation().await {
        if let Ok(cfg) = cache.get_config() {
            let _ = hyprland::apply_orientation(&initial, cfg).await;
        }
    }

    let mut stream = proxy.receive_accelerometer_orientation_changed().await;
    info!("Waiting for sensor events...");

    while let Some(property) = stream.next().await {
        if let Ok(orientation) = property.get().await {
            info!("Orientation changed: {}", orientation);
            
            match cache.get_config() {
                Ok(cfg) => {
                    if let Err(e) = hyprland::apply_orientation(&orientation, cfg).await {
                        error!("Error applying orientation: {}", e);
                    }
                }
                Err(e) => error!("Configuration error: {}", e),
            }
        }
    }

    Ok(())
}
