// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `JoinAccept`
//! For information about the protocol please go to the following [url](https://Sietse2202.github.io/crypts-of-the-lost/).

#![expect(missing_docs)]

use bevy::ecs::event::Event;
use std::net::SocketAddr;

/// Event from the server to the client whose join command got accepted
#[derive(serde::Deserialize, serde::Serialize, Debug, Eq, PartialEq, Clone, Event)]
pub struct JoinAccept {
    pub ip: SocketAddr,
    pub uuid: u64,
}

impl crate::event::Event for JoinAccept {}

impl crate::target::Targetable for JoinAccept {
    fn get_target(&self) -> &crate::target::Target {
        &crate::target::Target::Everyone
    }
}
