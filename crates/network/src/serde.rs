// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Serde
//! This module defines the messages and responses the server and client will send to each other.
//! This is in the form of two structs, [`Message`], and [`Response`], Message being
//! Server -> Client, and Response being Server <- Client.

use serde::{Deserialize, Serialize};

/// Message from the server, to the client
#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
#[non_exhaustive]
pub struct Message {}

/// Response from the client to the server
#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Response {}
