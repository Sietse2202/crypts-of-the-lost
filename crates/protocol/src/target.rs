// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Target
//! Defines the target type that tells to whom the `Event`
//! should be sent to.

use crate::event::EventKind;
use std::collections::HashSet;

/// The target(s) to send the event to.
#[derive(serde::Deserialize, serde::Serialize, Debug, Eq, PartialEq, Clone)]
pub enum Target {
    /// Sends the event to everyone
    Everyone,
    /// Sends the event to only one connection
    Player(u64),
    /// Sends the event to a group of connections
    Group(HashSet<u64>),
    /// Sends the event to all but one
    EveryoneExcept(u64),
    /// Sends the event to all but a group of connections
    EveryoneExceptGroup(HashSet<u64>),
}

impl Target {
    /// Checks if the provided id is a valid target
    #[must_use]
    pub fn is_recipient(&self, other: &u64) -> bool {
        if *other == 0 {
            return false;
        }

        match self {
            Self::Everyone => true,
            Self::EveryoneExceptGroup(ids) => !ids.contains(other),
            Self::EveryoneExcept(id) => id != other,
            Self::Group(ids) => ids.contains(other),
            Self::Player(id) => id == other,
        }
    }
}

/// no doc yet
#[enum_dispatch::enum_dispatch(EventKind)]
pub trait Targetable {
    /// Returns the [`Target`] associated with this type
    fn get_target(&self) -> Target;

    /// Returns whether the given id is a recipient according to the [`Target`].
    fn is_recipient(&self, other: &u64) -> bool {
        self.get_target().is_recipient(other)
    }
}
