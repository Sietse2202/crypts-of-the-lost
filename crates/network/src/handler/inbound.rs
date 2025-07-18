// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use super::NetworkHandler;
use crate::envelope::InboundMessage;
use quinn::ReadExactError;
use tracing::error;

type RecvResult = Option<Result<Vec<u8>, ReadExactError>>;

impl NetworkHandler {
    pub(super) async fn process_inbound(
        dispatcher_tx: flume::Sender<InboundMessage>,
        mut conn_rx: quinn::RecvStream,
    ) {
        let id = conn_rx.id();
        while let Some(data) = Self::receive_command(&mut conn_rx).await {
            let Ok(data) = data else {
                continue;
            };
            let Ok(cmd) = Self::deserialize_command(&data) else {
                error!(
                    "[Stream {id}] wasn't able to deserialize following data to `Command`: {data:?}"
                );
                continue;
            };
            let msg = InboundMessage::new(cmd);
            if let Err(e) = dispatcher_tx.send_async(msg).await {
                error!("[Stream {id}] failed to send data to dispatcher: {e}");
            }
        }
    }

    async fn receive_command(stream: &mut quinn::RecvStream) -> RecvResult {
        let mut len_buf = [0u8; 4];
        if let Err(e) = Self::read_exact(stream, &mut len_buf).await {
            return e;
        }

        let len = u32::from_be_bytes(len_buf);
        let mut data = vec![0u8; len as usize];
        if let Err(e) = Self::read_exact(stream, &mut data).await {
            return e;
        }

        Some(Ok(data))
    }

    async fn read_exact(stream: &mut quinn::RecvStream, buf: &mut [u8]) -> Result<(), RecvResult> {
        let id = stream.id();
        match stream.read_exact(buf).await {
            Ok(()) => Ok(()),
            Err(ReadExactError::FinishedEarly(n)) => {
                error!("[Stream {id}] EOF ({n} bytes read)");
                Err(None)
            }
            Err(e) => {
                error!("[Stream {id}] error during read: {e}");
                Err(Some(Err(e)))
            }
        }
    }
}
