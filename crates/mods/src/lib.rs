// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `mods`
//! This crate aims to add support for modding of the game. It provides this support via a bevy
//! [`Plugin`]
//!
//! [`Plugin`]: Plugin

pub(crate) mod toml;
pub(crate) mod rhai;

use bevy::app::App;
use bevy::prelude::Plugin;
use tracing::error;
use crate::toml::get_mods;

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
    #[expect(clippy::unwrap_used, clippy::todo)]
    fn build(&self, _app: &mut App) {
        let mods = get_mods();
        if let Err(e) = mods {
            error!("Failed to load mods: {}", e);
            return;
        }

        // PANIC: if it is Err, we have already returned
        for _mod in get_mods().unwrap() {
            todo!()
        }
    }
}
