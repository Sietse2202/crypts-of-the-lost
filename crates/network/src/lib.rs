// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Networking
//! This crate handles all networking code between the client and the server, this also
//! specifies the protocol they use to communicate.

#![expect(clippy::multiple_crate_versions)]

mod cert;
mod envelope;
mod error;
mod handler;
mod target;

pub use cert::Certs;
pub use error::{CertsError, HandlerError};
pub use handler::NetworkHandler;
pub use target::Target;
