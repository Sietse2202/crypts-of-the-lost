// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Envelope
//! This module wraps [`protocol::event::EventType`] and [`protocol::command::CommandType`] in
//! [`OutboundMessage`] and [`InboundMessage`], adding context and metadata
//! required by the networking layer.

mod inbound;
mod outbound;

pub use inbound::InboundMessage;
pub use outbound::OutboundMessage;
