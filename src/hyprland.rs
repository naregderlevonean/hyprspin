use crate::config::LuaConfig;
use anyhow::{Context, Result};
use log::error;

use hyprland::data::Monitors;
use hyprland::keyword::Keyword;
use hyprland::shared::{HyprData, HyprDataVec};

fn get_transform(orient: &str) -> Option<u8> {
    match orient {
        "normal" => Some(0),
        "left-up" => Some(1),
        "bottom-up" | "inverted" => Some(2),
        "right-up" => Some(3),
        _ => None,
    }
}

pub async fn apply_orientation(orientation: &str, config: &LuaConfig) -> Result<()> {
    let transform = match get_transform(orientation) {
        Some(t) => t,
        None => return Ok(()),
    };

    let monitors_list = Monitors::get()
        .context("Failed to get monitors")?
        .to_vec();

    let internal_monitor = monitors_list
        .iter()
        .find(|m| m.name.starts_with("eDP") || m.name.starts_with("LVDS"))
        .map(|m| m.name.clone())
        .unwrap_or_else(|| "eDP-1".to_string());

    for mon in &monitors_list {
        if mon.name == internal_monitor {
            let mon_cmd = format!(
                "{},{}x{}@{},{}x{},{},transform,{}",
                mon.name, mon.width, mon.height, mon.refresh_rate, mon.x, mon.y, mon.scale, transform
            );
            
            if let Err(e) = Keyword::set("monitor", mon_cmd) {
                error!("Monitor rotation failed for {}: {}", mon.name, e);
            }
        }

        match config.evaluate_spin(orientation, &mon.name) {
            Ok(actions) => {
                for action in actions {
                    if let Err(e) = action.execute() {
                        error!("Action failed: {} - {}", action.action, e);
                    }
                }
            }
            Err(e) => error!("Lua error: {}", e),
        }
    }

    let transform_str = transform.to_string();
    let _ = Keyword::set("input:touchdevice:transform", transform_str.clone());
    let _ = Keyword::set("input:touchdevice:output", internal_monitor.clone());
    let _ = Keyword::set("input:tablet:transform", transform_str);
    let _ = Keyword::set("input:tablet:output", internal_monitor);

    Ok(())
}
