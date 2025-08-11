// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `mods`
//! This crate aims to add support for modding of the game. It provides this support via a bevy
//! [`Plugin`]
//!
//! [`Plugin`]: Plugin

#![expect(clippy::multiple_crate_versions)]

pub(crate) mod data_driven;
pub(crate) mod rhai;
pub(crate) mod toml;

use crate::rhai::engine::get_default_engine;
use crate::toml::{DependencyData, ModData, get_mods};
use ::rhai::{Engine, ImmutableString};
use bevy::app::App;
use bevy::prelude::Plugin;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use tracing::error;

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
    #[expect(clippy::unwrap_used, clippy::cognitive_complexity)]
    fn build(&self, app: &mut App) {
        let mods = get_mods();
        if let Err(e) = mods {
            error!("Failed to load mods: {}", e);
            return;
        }

        // PANIC: if it is Err, we have already returned
        let mods = mods.unwrap();

        let mut hashes = Vec::with_capacity(mods.len());

        for mod_data in &mods {
            if let Err(e) = check_dependencies(mod_data, &mods) {
                error!("Failed loading mods: \"{e}\", aborting all mod loading");
                return;
            }

            let hash = paq::hash_source(&mod_data.path, false).to_string();
            hashes.push(hash);
        }

        let mod_data_list = Rc::new(
            mods.iter()
                .zip(hashes.iter())
                .map(|(m, hash)| {
                    (
                        m.toml_data.data.name.clone().into(),
                        m.toml_data.data.version.clone().into(),
                        hash.into(),
                    )
                })
                .collect::<HashSet<(ImmutableString, ImmutableString, ImmutableString)>>(),
        );

        for mod_data in &mods {
            let mod_data_list_clone = mod_data_list.clone();

            let permitted_mods: HashSet<(ImmutableString, ImmutableString, ImmutableString)> =
                mod_data
                    .toml_data
                    .dependencies
                    .clone()
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|(_, dep)| dep.optional)
                    .map(|(name, dep)| (name.into(), dep.version.into(), dep.checksum.into()))
                    .collect();

            let is_mod_enabled = Box::new(
                move |name: ImmutableString,
                      version: ImmutableString,
                      checksum: ImmutableString|
                      -> Option<bool> {
                    if !permitted_mods.contains(&(name.clone(), version.clone(), checksum.clone()))
                    {
                        return None;
                    }

                    Some(mod_data_list_clone.contains(&(name, version, checksum)))
                },
            );

            let engine = get_default_engine(mod_data.toml_data.data.name.as_str(), is_mod_enabled);

            if let Err(e) = run_mod(app, mod_data, &engine) {
                error!("Failed to run mod: {}", e);
            }
        }
    }
}

fn check_dependencies(
    mod_data: &ModData,
    loaded_mods: &[ModData],
) -> Result<(), Box<dyn std::error::Error>> {
    if semver::Version::parse(mod_data.toml_data.data.version.as_str()).is_err() {
        Err("Invalid version")?;
    }

    let mod_name: &str = &mod_data.toml_data.data.name;

    let mut hash_cache = HashMap::new();

    for (dep_name, dep_data) in mod_data.toml_data.dependencies.clone().unwrap_or_default() {
        if !dependencies_match(&dep_name, &dep_data, loaded_mods, &mut hash_cache) {
            let msg = format!(
                "Mod `{}` requires mod `{}`, but it is not loaded",
                mod_name, &dep_name
            );
            Err(msg)?;
        }
    }

    for (conflict_name, conflict_versions) in
        mod_data.toml_data.conflicts.clone().unwrap_or_default()
    {
        if conflict_occurs(&conflict_name, &conflict_versions, loaded_mods) {
            let msg = format!("Mod `{}` conflicts with mod `{}`", mod_name, &conflict_name);
            Err(msg)?;
        }
    }

    Ok(())
}

fn dependencies_match(
    dependency_name: &str,
    dependency_data: &DependencyData,
    loaded_mods: &[ModData],
    hash_cache: &mut HashMap<(String, String), String>,
) -> bool {
    if dependency_data.optional {
        return true;
    }

    let Ok(dep_version) = semver::Version::parse(&dependency_data.version) else {
        return false;
    };

    loaded_mods.iter().any(|mod_data| {
        let Ok(version) = semver::Version::parse(&mod_data.toml_data.data.version) else {
            return false;
        };

        let cache_key = (
            mod_data.toml_data.data.name.clone(),
            mod_data.toml_data.data.version.clone(),
        );

        let hash = hash_cache
            .entry(cache_key)
            .or_insert_with(|| paq::hash_source(&mod_data.path, false).to_string())
            .clone();

        let names_match = dependency_name == mod_data.toml_data.data.name;
        let versions_match = dep_version == version;
        let hashes_match = hash == dependency_data.checksum;

        names_match && versions_match && hashes_match
    })
}

fn conflict_occurs(conflict_name: &str, conflict_versions: &str, loaded_mods: &[ModData]) -> bool {
    let Ok(conflict_versions) = semver::VersionReq::parse(conflict_versions) else {
        return false;
    };

    loaded_mods.iter().any(|mod_data| {
        let Ok(version) = semver::Version::parse(&mod_data.toml_data.data.version) else {
            return false;
        };

        let names_match = conflict_name == mod_data.toml_data.data.name;
        let versions_match = conflict_versions.matches(&version);

        names_match && versions_match
    })
}

#[expect(clippy::todo)]
fn run_mod(
    _app: &App,
    _mod_data: &ModData,
    _engine: &Engine,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!();
}
