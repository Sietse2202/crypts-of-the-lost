// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Target
//! Defines the target type that tells to whom the `Event`
//! should be sent to.

use std::collections::HashSet;

/// The target(s) to send the event to.
#[derive(bincode::Encode, bincode::Decode, Debug, Eq, PartialEq, Clone)]
pub enum Target {
    /// Sends the event to everyone
    All,
    /// Sends the event to only one connection
    One(u64),
    /// Sends the event to a group of connections
    Group(HashSet<u64>),
    /// Sends the event to all but one
    AllButOne(u64),
    /// Sends the event to all but a group of connections
    AllBut(HashSet<u64>),
}

impl Target {
    /// Checks if the provided id is a valid target
    #[must_use]
    pub fn is_recipient(&self, other: &u64) -> bool {
        match self {
            Self::All => true,
            Self::AllBut(ids) => !ids.contains(other),
            Self::AllButOne(id) => id != other,
            Self::Group(ids) => ids.contains(other),
            Self::One(id) => id == other,
        }
    }
}
