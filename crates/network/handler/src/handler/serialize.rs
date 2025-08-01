// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use super::NetworkHandler;
use bincode::error::EncodeError;
use protocol::event::EventKind;

impl NetworkHandler {
    /// Serializes the `Event` struct to a `Vec<u8>`
    pub(super) fn serialize_event(event: EventKind) -> Result<Vec<u8>, EncodeError> {
        bincode::encode_to_vec(event, bincode::config::standard())
    }
}
