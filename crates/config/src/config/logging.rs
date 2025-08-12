// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `Logging`
//! Defines the Config used for logging.

/// The config used for setting up logging
#[derive(Debug, Clone, serde::Deserialize, Default)]
pub struct LoggingConfig {
    /// The output formatting
    #[serde(default)]
    pub output_format: OutputFormat,
    /// The maximum log level for the output
    #[serde(default)]
    pub log_level: LogLevel,
}

/// The formatting of the output.
#[derive(Debug, Clone, serde::Deserialize)]
pub enum OutputFormat {
    /// Default
    Default,
    /// Human-readable
    Pretty,
    /// Machine-readable JSON
    Json,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Default
    }
}

/// The log level
#[derive(Debug, Clone, serde::Deserialize)]
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

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}
