// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Join
//! For information about the protocol please go to the following [url](https://Sietse2202.github.io/crypts-of-the-lost/).

#![expect(missing_docs)]

use bevy::ecs::event::Event;
use std::net::SocketAddr;

/// The command sent to the server after successful connection to it.
#[derive(
    serde::Deserialize,
    serde::Serialize,
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
    pub uuid: u64,
    pub hash: u64,
    pub ip: Option<SocketAddr>, // needed for the network handler
}
