use anyhow::{Context, Result};
use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::keyword::Keyword;
use mlua::prelude::*;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Action {
    pub delay_ms: u64,
    pub action: String,
    pub args: String,
}

impl Action {
    pub fn execute(&self) -> Result<()> {
        match self.action.as_str() {
            "exec" => {
                Command::new("sh").arg("-c").arg(&self.args).spawn()?;
            }
            "keyword" => {
                if let Some((k, v)) = self.args.split_once(' ') {
                    Keyword::set(k, v)?;
                } else {
                    Keyword::set(&self.args, "")?;
                }
            }
            "dispatch" => {
                if let Some((d, a)) = self.args.split_once(' ') {
                    Dispatch::call(DispatchType::Custom(d, a))?;
                } else {
                    Dispatch::call(DispatchType::Custom(&self.args, ""))?;
                }
            }
            _ => {
                Dispatch::call(DispatchType::Custom(&self.action, &self.args))?;
            }
        }
        Ok(())
    }
}

pub struct LuaConfig {
    lua: Lua,
}

impl LuaConfig {
    pub fn new(path: &Path) -> Result<Self> {
        let lua = Lua::new();
        let script = read_to_string(path)?;

        lua.load(&script).exec()?;
        Ok(Self { lua })
    }

    pub fn evaluate_spin(&self, orientation: &str, monitor: &str) -> Result<Vec<Action>> {
        let globals = self.lua.globals();
        let on_spin: LuaFunction = globals.get("on_spin")?;

        let ctx = self.lua.create_table()?;
        ctx.set("orientation", orientation)?;
        ctx.set("monitor", monitor)?;

        let res: Option<LuaTable> = on_spin.call(ctx)?;
        Ok(Self::parse_actions(res))
    }

    fn parse_actions(res: Option<LuaTable>) -> Vec<Action> {
        let mut actions = Vec::new();
        if let Some(t) = res {
            if t.contains_key("action").unwrap_or(false) {
                let delay_ms = t.get("delay").unwrap_or(0);
                let action: String = t.get("action").unwrap_or_default();
                let args: String = t.get("args").unwrap_or_default();
                actions.push(Action { delay_ms, action, args });
            } else {
                for pair in t.pairs::<i64, LuaTable>() {
                    if let Ok((_, action_table)) = pair {
                        let delay_ms = action_table.get("delay").unwrap_or(0);
                        let action: String = action_table.get("action").unwrap_or_default();
                        let args: String = action_table.get("args").unwrap_or_default();
                        actions.push(Action { delay_ms, action, args });
                    }
                }
            }
        }
        actions
    }
}
