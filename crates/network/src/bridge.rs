// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Dispatcher
//! This module handles the setup of the network and the communication
//! between the handler and the bevy event system.

mod command_receiver;
mod event_sender;

pub use command_receiver::{CommandReceiver, process_incoming_commands};
pub use event_sender::{EventSender, process_outbound_events};
