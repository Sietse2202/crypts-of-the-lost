//! # `mods`
//! This crate aims to add support for modding of the game. It provides this support via a bevy
//! [`Plugin`]
//!
//! [`Plugin`]: Plugin
use bevy::app::App;
use bevy::prelude::Plugin;

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
    fn build(&self, _app: &mut App) {}
}
