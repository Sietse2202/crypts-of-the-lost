// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use super::NetworkHandler;
use protocol::event::EventKind;
use rmp_serde::{encode, to_vec};

impl NetworkHandler {
    /// Serializes the `Event` struct to a `Vec<u8>`
    pub(super) fn serialize_event(event: &EventKind) -> Result<Vec<u8>, encode::Error> {
        to_vec(&event)
    }
}
