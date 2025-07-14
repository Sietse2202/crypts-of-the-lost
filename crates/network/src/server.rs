// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Server
//! This module aims to provide an API for dealing with server-client connections.

use bincode::config::standard;
use bincode::serde::{decode_from_reader, encode_to_vec};
use quinn::{RecvStream, SendStream};
use serde::{Deserialize, Serialize};
use std::io::BufReader;

/// Sends the message to a client
///
/// # Errors
/// The function errors if one of the following holds true
/// - The message can't be encoded
/// - The message can't be written to the stream
/// - The length cannot be cast to a u32
pub async fn send_message<T: Serialize + Send + Sync>(
    send: &mut SendStream,
    message: &T,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let serialized = encode_to_vec(message, standard())?;
    let len = u32::try_from(serialized.len())?;

    // Send length first (4 bytes)
    send.write_all(&len.to_be_bytes()).await?;
    send.write_all(&serialized).await?;

    Ok(())
}

/// Parses a response from the client
///
/// # Errors
/// The function errors if one of the following holds true
/// - Reading the buffer fails
/// - Decoding the message fails
pub async fn receive_message<T: for<'de> Deserialize<'de>>(
    recv: &mut RecvStream,
) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
    // Read length first (4 bytes)
    let mut len_bytes = [0u8; 4];
    recv.read_exact(&mut len_bytes).await?;
    let len = u32::from_be_bytes(len_bytes) as usize;

    let mut buffer = vec![0u8; len];
    recv.read_exact(&mut buffer).await?;

    let message = decode_from_reader(BufReader::new(&*buffer), standard())?;
    Ok(message)
}
