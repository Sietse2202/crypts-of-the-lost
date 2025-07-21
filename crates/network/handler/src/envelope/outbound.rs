// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use crate::target::Target;
use protocol::event::Event;

/// This struct contains the event waiting to be sent by the server
/// and some extra metadata and context about it and its target.
#[derive(Debug, Clone)]
pub struct OutboundMessage {
    pub target: Target,
    pub event: Event,
}

impl OutboundMessage {
    /// Creates a new instance of [`OutboundMessage`]
    #[inline]
    #[must_use]
    pub const fn new(target: Target, event: Event) -> Self {
        Self { target, event }
    }
}
