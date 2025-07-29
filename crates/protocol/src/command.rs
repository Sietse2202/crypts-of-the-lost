// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Command
//! This module contains all types used for the communication from the client
//! to the server.

pub mod join;

use bincode::{Decode, Encode};

/// Command from the client to the server
#[derive(Encode, Decode, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Command {
    /// The type of command it is
    pub command_type: CommandType,
}

/// The type of command from the client to the server
#[derive(Encode, Decode, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
#[non_exhaustive]
pub enum CommandType {
    /// New connection
    Join(join::Join),
}
