// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! This module specifies the Command type used for requests by the client to the server.

use serde::{Deserialize, Serialize};

/// Command from the client to the server
#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
#[non_exhaustive]
pub struct Command {}
