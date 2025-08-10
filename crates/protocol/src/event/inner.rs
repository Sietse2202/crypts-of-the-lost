// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `JoinAccept`
//! For information about the protocol please go to the following [url](https://Sietse2202.github.io/crypts-of-the-lost/).

use crate::target::Target;

/// Additional metadata which every event needs
#[derive(serde::Deserialize, serde::Serialize, Debug, Eq, PartialEq, Clone)]
pub struct EventInner {
    target: Target,
}
