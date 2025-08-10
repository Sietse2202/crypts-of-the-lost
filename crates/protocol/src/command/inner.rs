// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `CommandInner`
//! For information about the protocol please go to the following [url](https://Sietse2202.github.io/crypts-of-the-lost/).

/// Additional metadata which every command needs
#[derive(
    serde::Deserialize, serde::Serialize, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash,
)]
pub struct CommandInner {}
