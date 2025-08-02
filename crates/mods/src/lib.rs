// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `mods`
//! This crate aims to add support for modding of the game. It provides this support via a bevy
//! [`Plugin`]
//!
//! [`Plugin`]: Plugin

pub(crate) mod rhai;
pub(crate) mod toml;

use crate::toml::{ModData, get_mods};
use ::rhai::{Engine, ImmutableString};
use bevy::app::App;
use bevy::prelude::Plugin;
use std::sync::Arc;
use tracing::{error, info, warn};

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

        for mod_data in &mods {
            if let Err(e) = run_mod(app, mod_data) {
                error!("Failed to run mod: {}", e);
            }
        }
    }
}

fn run_mod(_app: &App, mod_data: &ModData) -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = Engine::new();

    let name: Arc<str> = Arc::from(mod_data.toml_data.data.name.as_str());

    let name_clone = Arc::clone(&name);
    engine.register_fn("info", move |msg: ImmutableString| {
        info!("Mod \"{}\": {msg}", name_clone);
    });

    let name_clone = Arc::clone(&name);
    engine.register_fn("warn", move |msg: ImmutableString| {
        warn!("Mod \"{}\": {msg}", name_clone);
    });

    let name_clone = Arc::clone(&name);
    engine.register_fn("error", move |msg: ImmutableString| {
        error!("Mod \"{}\": {msg}", name_clone);
    });

    engine.run_with_scope(&mut rhai::scope::get_default_scope(), "")?;

    Ok(())
}
