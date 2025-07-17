// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use protocol::command::Command;

/// This struct contains the command received from the client and some extra
/// metadata and context about it and its source.
//#[expect(dead_code)]
#[derive(Debug)]
pub struct InboundMessage {
    pub command: Command,
}

impl InboundMessage {
    /// Creates a new instance of [`OutboundMessage`]
    #[inline]
    #[must_use]
    pub const fn new(command: Command) -> Self {
        Self { command }
    }
}
