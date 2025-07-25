// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `PlayerJoined`
//! For information about the protocol please go to the following [url](https://Sietse2202.github.io/crypts-of-the-lost/).

use crate::event::EventInner;

/// New player joined. Gets sent to everyone except the new player
#[derive(
    bincode::Encode, bincode::Decode, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash,
)]
pub struct PlayerJoined {
    inner: EventInner,
}
