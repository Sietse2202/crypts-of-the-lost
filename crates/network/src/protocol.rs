// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Protocol
//! This module defines the protocol the server and client will use to communicate
//! with each other.
//! This is in the form of two structs, [`Message`], and [`Response`], Message being
//! Server -> Client, and Response being Client -> Server.

mod command;
mod event;

pub use command::Command;
pub use event::Event;
