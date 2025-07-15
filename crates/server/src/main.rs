// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Server
//! This binary crate exists as the package containing the entire server side logic.
//! This includes but is not limited to:
//! - CLI logic
//! - The calling of functions from other crates in the workspace

#![expect(clippy::multiple_crate_versions)]

use clap::Parser;

#[derive(Parser, Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash, Default)]
struct Cli {}

fn main() {
    let _args = Cli::parse();
}
