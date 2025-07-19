// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Error
//! Defines the error type for the network crate

use thiserror::Error;

/// Error type used by the [`crate::handler::NetworkHandler`]
#[derive(Debug, Error)]
pub enum HandlerError {
    /// Error from IO
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Bincode `DecodeError`
    #[error("DecodeError: {0}")]
    Decode(#[from] bincode::error::DecodeError),
    /// Bincode `EncodeError`,
    #[error("EncodeError: {0}")]
    Encode(#[from] bincode::error::EncodeError),
    /// `ConnectionError` from quinn
    #[error("ConnectionError: {0}")]
    Connection(#[from] quinn::ConnectionError),
}
