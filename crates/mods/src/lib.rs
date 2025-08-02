// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `mods`
//! This crate aims to add support for modding of the game. It provides this support via a bevy
//! [`Plugin`]
//!
//! [`Plugin`]: Plugin

pub(crate) mod data_driven;
pub(crate) mod rhai;
pub(crate) mod toml;

use crate::rhai::{MAX_SCRIPT_OPS, SCRIPT_WARN_OPS};
use crate::toml::{ModData, get_mods};
use ::rhai::{Dynamic, Engine, ImmutableString};
use bevy::app::App;
use bevy::prelude::Plugin;
use std::sync::Arc;
use tracing::{error, info, warn};
use crate::rhai::engine::get_default_engine;

/// The mod directory in relation to the server root folder
pub const MOD_DIR: &str = "mods";

/// The name of the toml file of each mod
pub const MOD_CONFIG_FILE: &str = "mod.toml";

/// # Modding Plugin
/// Adds modding support to the main server bevy app.
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use mods::ModdingPlugin;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(ModdingPlugin)
///     .run();
/// ```
#[derive(Default, Debug, Copy, Clone, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub struct ModdingPlugin;

impl Plugin for ModdingPlugin {
    #[expect(clippy::unwrap_used)]
    fn build(&self, app: &mut App) {
        let mods = get_mods();
        if let Err(e) = mods {
            error!("Failed to load mods: {}", e);
            return;
        }

        // PANIC: if it is Err, we have already returned
        let mods = mods.unwrap();

        'load_mod: for mod_data in &mods {
            if let Err(e) = check_dependencies(mod_data, mods.iter()) {
                error!("{}", e);
                break 'load_mod;
            }

            let mut engine = get_default_engine(mod_data.toml_data.data.name.as_str());

            if let Err(e) = run_mod(app, mod_data, &mut engine) {
                error!("Failed to run mod: {}", e);
            }
        }
    }
}

fn check_dependencies<'a, 'b, 'c>(
    mod_data: &'a ModData,
    mods: impl Iterator<Item = &'b ModData>,
) -> Result<(), &'c str> {
    if let Some(deps) = &mod_data.toml_data.dependencies {
        for (name, data) in deps {
            if data.optional {
                continue;
            }

            if !mods.iter().any(|m| {
                m.toml_data.data.name.as_str() == name && m.toml_data.data.version == data.version
            }) {
                return Err(&format!(
                    "Mod `{}` depends on mod `{}`, but it is not loaded",
                    mod_data.toml_data.data.name.as_str(),
                    name
                ));
            }
        }
    }

    Ok(())
}

fn run_mod(
    _app: &App,
    _mod_data: &ModData,
    engine: &mut Engine,
) -> Result<(), Box<dyn std::error::Error>> {
    engine.run_with_scope(&mut rhai::scope::get_default_scope(), "")?;

    Ok(())
}
