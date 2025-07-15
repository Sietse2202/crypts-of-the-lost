// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Dispatcher
//! This module defines types that are used to communicate between client and server.

#![allow(dead_code)]

use bevy_ecs::prelude::Resource;
use quinn::{Connection, Endpoint, ServerConfig};
use std::collections::VecDeque;
use std::fmt::Formatter;
use std::net::SocketAddr;

pub type Data = Box<[u8]>;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub enum NetworkEvent {
    NewConnection(SocketAddr),
    ConnectionClosed(SocketAddr),
    Message(Response),
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub struct Response {
    client: SocketAddr,
    data: Data,
}

pub trait NetworkTarget {
    fn receivers(&self, clients: Vec<SocketAddr>) -> Vec<SocketAddr>;
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub struct Single(SocketAddr);

impl NetworkTarget for Single {
    fn receivers(&self, _: Vec<SocketAddr>) -> Vec<SocketAddr> {
        vec![self.0]
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub struct Group(Vec<SocketAddr>);

impl NetworkTarget for Group {
    fn receivers(&self, _: Vec<SocketAddr>) -> Vec<SocketAddr> {
        self.0.clone()
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    Ord,
    PartialOrd,
    Default,
)]
pub struct All;

impl NetworkTarget for All {
    fn receivers(&self, clients: Vec<SocketAddr>) -> Vec<SocketAddr> {
        clients
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub struct AllBut(SocketAddr);

impl NetworkTarget for AllBut {
    fn receivers(&self, clients: Vec<SocketAddr>) -> Vec<SocketAddr> {
        clients.into_iter().filter(|c| c != &self.0).collect()
    }
}

pub struct MessageData {
    target: Box<dyn NetworkTarget + Send + Sync>,
    data: Data,
}

impl std::fmt::Debug for MessageData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageData")
            .field("target", &"Box<dyn NetworkTarget>")
            .field("data", &self.data)
            .finish()
    }
}

#[derive(Debug, Resource)]
pub struct NetworkDispatcher {
    socket: SocketAddr,
    clients: Vec<SocketAddr>,
    pub(crate) message_queue: VecDeque<MessageData>,
    pub(crate) event_queue: VecDeque<NetworkEvent>,
}

impl NetworkDispatcher {
    pub const fn new(addr: SocketAddr) -> Self {
        Self {
            message_queue: VecDeque::new(),
            event_queue: VecDeque::new(),
            clients: Vec::new(),
            socket: addr,
        }
    }

    pub const fn socket(&self) -> SocketAddr {
        self.socket
    }

    pub fn clients(&self) -> &[SocketAddr] {
        &self.clients
    }

    pub async fn listen<F>(self, config: ServerConfig, handle_conn: F) -> anyhow::Result<()>
    where
        F: AsyncFn(&Self, Connection) -> anyhow::Result<()> + Send + Sync + 'static,
    {
        let endpoint = Endpoint::server(config, self.socket)?;
        let certs = crate::cert::Certs::read_from_file()?;
        let (certs, key) = (certs.certs(), certs.key());

        while let Some(conn) = endpoint.accept().await {
            let connection = conn.await?;

            handle_conn(&self, connection).await?;
        }

        Ok(())
    }
}
