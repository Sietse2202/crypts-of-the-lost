//! This crate provides an abstraction over whom to send a message to. This is in the form of a
//! trait

use std::collections::HashSet;
use std::net::SocketAddr;

/// Abstraction over a network target, this trait is used to select the clients a message should be
/// sent to
pub trait NetworkTarget {
    /// This method should return the clients to send a message to.
    fn receivers(&self, clients: HashSet<SocketAddr>) -> HashSet<SocketAddr>;
}

/// Target to send a message to a single client
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Single(SocketAddr);

impl Single {
    /// Create a new self from the address
    #[inline]
    #[must_use]
    pub const fn new(client: SocketAddr) -> Self {
        Self(client)
    }
}

impl NetworkTarget for Single {
    fn receivers(&self, _: HashSet<SocketAddr>) -> HashSet<SocketAddr> {
        let mut set = HashSet::new();
        set.insert(self.0);
        set
    }
}

/// Network target, to send a message to a group of people
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group(HashSet<SocketAddr>);

impl Group {
    /// Create a new group from an iterator
    #[inline]
    #[must_use]
    pub fn new<I, Item>(ground: I) -> Self
    where
        I: IntoIterator<Item = Item>,
        Item: Into<SocketAddr>,
    {
        Self(ground.into_iter().map(Into::into).collect())
    }
}

impl NetworkTarget for Group {
    fn receivers(&self, _: HashSet<SocketAddr>) -> HashSet<SocketAddr> {
        self.0.clone()
    }
}

/// Target to send a message to everyone
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct All;

impl NetworkTarget for All {
    fn receivers(&self, clients: HashSet<SocketAddr>) -> HashSet<SocketAddr> {
        clients
    }
}

/// Sends a message to everyone but one client
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct AllBut(pub(crate) SocketAddr);

impl AllBut {
    /// Creates a new [`AllBut`] instance from an address
    #[inline]
    #[must_use]
    pub const fn new(addr: SocketAddr) -> Self {
        Self(addr)
    }
}

impl NetworkTarget for AllBut {
    fn receivers(&self, clients: HashSet<SocketAddr>) -> HashSet<SocketAddr> {
        clients.into_iter().filter(|c| c != &self.0).collect()
    }
}
