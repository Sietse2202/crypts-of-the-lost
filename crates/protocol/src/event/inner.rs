// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Inner
//! This module defines the [`EventInner`] type for the actual events
//! for an event message from the server.

use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
#[non_exhaustive]
pub enum EventInner {}
