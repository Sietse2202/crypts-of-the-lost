// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Logging
//! This module sets up the logging.

use config::config::logging::{LogLevel, LoggingConfig, OutputFormat};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn setup_logging(config: &LoggingConfig) {
    let mut subscriber_builder = FmtSubscriber::builder();

    // set max log level
    subscriber_builder = match config.log_level {
        LogLevel::Trace => subscriber_builder.with_max_level(Level::TRACE),
        LogLevel::Debug => subscriber_builder.with_max_level(Level::DEBUG),
        LogLevel::Info => subscriber_builder.with_max_level(Level::INFO),
        LogLevel::Warn => subscriber_builder.with_max_level(Level::WARN),
        LogLevel::Error => subscriber_builder.with_max_level(Level::ERROR),
    };

    // set output format
    match config.output_format {
        OutputFormat::Default => subscriber_builder.init(),
        OutputFormat::Pretty => subscriber_builder.pretty().init(),
        OutputFormat::Json => subscriber_builder.json().init(),
    }
}
