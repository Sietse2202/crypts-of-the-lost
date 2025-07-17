// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Command
//! This module contains all types used for the communication from the client
//! to the server.

mod inner;
use inner::CommandInner;

use bincode::{Decode, Encode};

/// Command from the client to the server
#[derive(Encode, Decode, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
#[non_exhaustive]
pub struct Command {
    inner: CommandInner,
}
