//! # Server
//! This binary crate exists as the package containing the entire server side logic.
//! This includes but is not limited to:
//! - CLI logic
//! - The calling of functions from other crates in the workspace
//!
//! ---
//!
//! ```
//! Copyright (C) 2025  Crypts of the Lost Team
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU Affero General Public License as
//! published by the Free Software Foundation, either version 3 of the
//! License, or (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU Affero General Public License for more details.
//!
//! You should have received a copy of the GNU Affero General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.
//! ```

use clap::Parser;

#[derive(Parser, Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash, Default)]
struct Cli {}

fn main() {
    let _args = Cli::parse();
}
