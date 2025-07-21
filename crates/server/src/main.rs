// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Server
//! This binary crate exists as the package containing the entire server side logic.
//! This includes but is not limited to:
//! - CLI logic
//! - The calling of functions from other crates in the workspace

#![expect(clippy::multiple_crate_versions)]

use bevy::prelude::*;
use config::parse_config;
use dispatcher::Network;
use protocol::Protocol;
use std::time::Duration;

const TPS: f64 = 16.;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = parse_config()?;

    App::new()
        .add_plugins(
            MinimalPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
                Duration::from_secs_f64(1. / TPS),
            )),
        )
        .add_plugins(Protocol)
        .add_plugins(Network)
        .insert_resource(config)
        .run();

    Ok(())
}
