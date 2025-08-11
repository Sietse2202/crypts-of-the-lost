// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use protocol::command::CommandKind;
use rmp_serde::{decode, from_slice};
use tracing::trace;

use super::NetworkHandler;

impl NetworkHandler {
    #[tracing::instrument]
    pub(super) fn deserialize_command(data: &[u8]) -> Result<CommandKind, decode::Error> {
        trace!("deserializing command");
        let cmd = from_slice(data)?;
        Ok(cmd)
    }
}
