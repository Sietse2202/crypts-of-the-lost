// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use super::Command;

/// This struct contains a
#[expect(dead_code)]
#[derive(Debug)]
pub struct InboundMessage {
    command: Command,
}

impl InboundMessage {
    /// Creates a new instance of [`OutboundMessage`]
    #[inline]
    #[must_use]
    pub const fn new(command: Command) -> Self {
        Self { command }
    }
}
