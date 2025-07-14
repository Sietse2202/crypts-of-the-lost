//! # xtask
//! This crate is the way we do CI; it is used to build, run, format, and check the code.

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
}

#[expect(clippy::print_stdout)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

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

            let taplo = Command::new("taplo")
                .args(["fmt"])
                .status()?;

            if !taplo.success() {
                std::process::exit(101);
            }
        }
    }

    Ok(())
}
