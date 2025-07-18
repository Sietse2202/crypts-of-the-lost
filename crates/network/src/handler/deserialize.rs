// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use bincode::error::DecodeError;
use protocol::command::Command;

use super::NetworkHandler;

impl NetworkHandler {
    pub(super) fn deserialize_command(data: &[u8]) -> Result<Command, DecodeError> {
        let (cmd, bytes_read) =
            bincode::decode_from_slice::<Command, _>(data, bincode::config::standard())?;
        debug_assert_eq!(bytes_read, data.len(), "Unexpected extra bytes in buffer");
        Ok(cmd)
    }
}
