// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Join
//! For information about the protocol please go to the following [url](https://Sietse2202.github.io/crypts-of-the-lost/).

use crate::command::CommandInner;
use bevy::ecs::event::Event;

/// The command sent to the server after successful connection to it.
#[derive(
    bincode::Encode,
    bincode::Decode,
    Debug,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Copy,
    Clone,
    Hash,
    Event,
)]
pub struct Join {
    inner: CommandInner,
}
