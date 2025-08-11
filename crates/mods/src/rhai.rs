// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

pub mod engine;
pub mod scope;

/// Max memory the engine can use before forcefully quiting the script in bytes
pub const MAX_SCRIPT_MEMORY: usize = 1024 * 1024 * 1024;
/// Number of ops the engine preforms before forcefully quitting the script.
pub const MAX_SCRIPT_OPS: u64 = 10_000;
/// Number of ops the engine preforms before warning,
/// the warning occurs every `SCRIPT_WARN_OPS` ops.
pub const SCRIPT_WARN_OPS: u64 = 1_000;
