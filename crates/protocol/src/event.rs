// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Event
//! This module contains all types used for the communication from the server
//! to the client.

mod inner;
use inner::EventInner;

use bincode::{Decode, Encode};

/// Message from the server, to the client
#[derive(Encode, Decode, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
#[non_exhaustive]
pub struct Event {
    inner: EventInner,
}
