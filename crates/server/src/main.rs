// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Server
//! This binary crate exists as the package containing the entire server side logic.

#![expect(clippy::multiple_crate_versions)]

mod logging;

use bevy::prelude::*;
use config::parse_config;
use logging::setup_logging;
use network::Network;
use protocol::Protocol;
use std::time::Duration;

const TPS: f64 = 16.;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();

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
