// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Error
//! Defines the error type for the network crate

use bincode::error::DecodeError;
use quinn::{ClosedStream, ReadError, WriteError};
use rustls_pki_types::pem;
use thiserror::Error;

/// Errors the dispatcher can encounter
#[derive(Error, Debug)]
pub enum DispatcherError {
    /// Io error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Quinn connection error
    #[error("connection error: {0}")]
    Connection(#[from] quinn::ConnectionError),
    /// Pem error from trying to get the certificates and key
    #[error("pem error: {0}")]
    Pem(#[from] pem::Error),
    /// An error while reading the buffer
    #[error("read error: {0}")]
    Read(#[from] ReadError),
    /// Basic write error
    #[error("write error: {0}")]
    Write(#[from] WriteError),
    /// Stream closed
    #[error("closed stream")]
    Closed(#[from] ClosedStream),
    /// Bincode decoding error
    #[error("bincode decode error: {0}")]
    Decode(#[from] DecodeError),
}

pub(crate) type Result<T> = std::result::Result<T, DispatcherError>;

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
}
