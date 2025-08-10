// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use protocol::command::Command;
use rmp_serde::{decode, from_slice};

use super::NetworkHandler;

impl NetworkHandler {
    pub(super) fn deserialize_command(data: &[u8]) -> Result<Command, decode::Error> {
        let cmd = from_slice(data)?;
        Ok(cmd)
    }
}
