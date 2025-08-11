// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Command
//! This module contains all types used for the communication from the client
//! to the server.

pub mod join;

/// Command from the client to the server
#[derive(
    serde::Deserialize, serde::Serialize, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash,
)]
#[enum_dispatch::enum_dispatch]
#[non_exhaustive]
pub enum CommandKind {
    /// New connection
    Join(join::Join),
}

/// Trait to be implemented by each command
#[enum_dispatch::enum_dispatch(CommandKind)]
pub trait Command {}
