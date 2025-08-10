// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `PlayerJoined`
//! For information about the protocol please go to the following [url](https://Sietse2202.github.io/crypts-of-the-lost/).

use bevy::ecs::event::Event;

/// New player joined. Gets sent to everyone except the new player
#[derive(serde::Deserialize, serde::Serialize, Debug, Eq, PartialEq, Clone, Event)]
pub struct PlayerJoined {}

impl crate::Event for PlayerJoined {}

impl crate::Targetable for PlayerJoined {
    fn get_target(&self) -> crate::Target {
        crate::Target::Everyone
    }
}
