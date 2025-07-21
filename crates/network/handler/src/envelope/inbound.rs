// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use protocol::command::Command;
use std::net::SocketAddr;

/// This struct contains the command received from the client and some extra
/// metadata and context about it and its source.
//#[expect(dead_code)]
#[derive(Debug)]
pub struct InboundMessage {
    pub source: SocketAddr,
    pub command: Command,
}

impl InboundMessage {
    /// Creates a new instance of [`OutboundMessage`]
    #[inline]
    #[must_use]
    pub const fn new(source: SocketAddr, command: Command) -> Self {
        Self { source, command }
    }
}
