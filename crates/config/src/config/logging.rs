// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `Logging`
//! Defines the Config used for logging.

/// The config used for setting up logging
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LoggingConfig {
    /// The output formatting
    pub output_format: OutputFormat,
    /// The maximum log level for the output
    pub log_level: LogLevel,
}

/// The formatting of the output.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum OutputFormat {
    /// Human-readable
    Pretty,
    /// Machine-readable JSON
    Json,
}

/// The log level
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum LogLevel {
    /// Lowest level, very verbose
    Trace,
    /// Lower priority information
    Debug,
    /// Useful information
    Info,
    /// Hazardous information
    Warn,
    /// Very serious errors
    Error,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::Pretty,
            log_level: LogLevel::Info,
        }
    }
}
