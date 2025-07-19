// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Target
//! Defines the target type that tells to whom the `Event`.
//! should be sent to.

/// The target(s) to send the event to.
#[derive(
    bincode::Encode, bincode::Decode, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash,
)]
pub enum ClientTarget {
    /// Sends the event to everyone
    All,
}
