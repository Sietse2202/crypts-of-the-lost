// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Logging
//! This module sets up the logging.

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[expect(clippy::expect_used)]
pub fn setup_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
