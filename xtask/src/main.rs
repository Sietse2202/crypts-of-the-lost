//! # xtask
//! This crate is the way we do CI; it is used to build, run, format, and check the code.
//!
//! ---
//!
//! ```md
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

use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser, Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
struct Cli {
    #[clap(subcommand)]
    sub_commands: SubCommands,
}

#[derive(Subcommand, Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
enum SubCommands {
    // Builds the server in *debug*
    Build,
    // This does all formatting checks, including toml files and the like
    Check,
    // Formats all code, including toml files
    Fmt,
    // Runs tests using nextest
    Test,
}

#[expect(clippy::print_stdout)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // Can be unsafe in multithreaded programs, but this won't be multithreaded, so it's good
    #[expect(unsafe_code)]
    unsafe {
        std::env::set_var("TAPLO_CONFIG", "./.config/taplo.toml");
    };

    // keeping it for later use
    #[expect(unused)]
    let is_ci: bool = std::env::var("IS_CI").is_ok();

    match args.sub_commands {
        SubCommands::Build => {
            let status = Command::new("cargo")
                .args([
                    "build",
                    "--release",
                    "--bin",
                    "server",
                    "--locked",
                    "--color=always",
                    "--quiet",
                ])
                .status()?;

            if !status.success() {
                std::process::exit(101);
            }
        }
        SubCommands::Check => {
            let commands: Vec<(&str, &[&str])> = vec![
                ("cargo", &["clippy", "--workspace", "--", "-Dwarnings"]),
                ("taplo", &["lint"]),
                ("cargo", &["fmt", "--", "--check"]),
                ("typos", &["--color", "always"]),
            ];

            for command in commands {
                let status = Command::new(command.0).args(command.1).status()?;

                if !status.success() {
                    std::process::exit(101);
                }
            }

            println!("\n\x1b[32;1m{:>12}\x1b[0m all checks", "Passed");
        }
        SubCommands::Fmt => {
            let fmt = Command::new("cargo")
                .args(["fmt", "--", "--all", "--", "--color=always"])
                .status()?;

            if !fmt.success() {
                std::process::exit(101);
            }

            let taplo = Command::new("taplo").args(["fmt"]).status()?;

            if !taplo.success() {
                std::process::exit(101);
            }
        }
        SubCommands::Test => {
            let nextest = Command::new("cargo")
                .args([
                    "nextest",
                    "run",
                    "--profile",
                    "ci",
                    "--release",
                    "--no-tests",
                    "pass",
                ])
                .status()?;

            if !nextest.success() {
                std::process::exit(101);
            }
        }
    }

    Ok(())
}
