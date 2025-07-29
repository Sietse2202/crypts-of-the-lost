// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `JoinAccept`
//! For information about the protocol, please go to the following
//! [url](https://Sietse2202.github.io/crypts-of-the-lost/).

/// Event from the server to the client whose join command got accepted
#[derive(
    bincode::Encode, bincode::Decode, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash,
)]
pub struct JoinAccept {}
