// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use crate::target::NetworkTarget;
use protocol::event::Event;

/// This struct contains the event waiting to be sent by the server
/// and some extra metadata and context about it and its target.
#[expect(dead_code)]
pub struct OutboundMessage {
    target: Box<dyn NetworkTarget + Send + Sync>,
    event: Event,
}

impl OutboundMessage {
    /// Creates a new instance of [`OutboundMessage`]
    #[inline]
    #[must_use]
    pub fn new(target: Box<dyn NetworkTarget + Send + Sync>, event: Event) -> Self {
        Self { target, event }
    }
}

impl std::fmt::Debug for OutboundMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OutboundMessage")
            .field("target", &"Box<dyn NetworkTarget>")
            .field("event", &self.event)
            .finish()
    }
}
