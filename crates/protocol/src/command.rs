// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Command
//! This module contains all types used for the communication from the client
//! to the server.

mod inner;
pub mod join;

pub use inner::CommandInner;

/// Command from the client to the server
#[derive(
    serde::Deserialize, serde::Serialize, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash,
)]
#[non_exhaustive]
pub enum Command {
    /// New connection
    Join(join::Join),
}
