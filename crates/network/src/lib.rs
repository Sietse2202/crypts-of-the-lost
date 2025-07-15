// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Networking
//! This crate handles all networking code between the client and the server, this also
//! specifies the interface they use to communicate.

#![expect(clippy::multiple_crate_versions)]
#![feature(never_type)]
#![feature(trivial_bounds)]

pub mod cert;
pub mod dispatcher;
pub mod serde;
