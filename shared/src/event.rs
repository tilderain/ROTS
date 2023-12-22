use bevy::prelude::*;

use serde::{Deserialize, Serialize};
use crate::netlib::Endpoint;

pub mod client;
pub mod server;
pub mod spells;

#[derive(Debug, Clone, Copy, Component, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NetEntId(pub u64);
impl NetEntId {
    pub fn random() -> Self {
        Self(rand::random())
    }
}

#[derive(Debug, Clone, Event)]
pub struct EventFromEndpoint<E> {
    pub event: E,
    pub endpoint: Endpoint,
}

/// Event Reader with endpoint data.
pub type ERFE<'w, 's, E> = EventReader<'w, 's, EventFromEndpoint<E>>;

impl<E> EventFromEndpoint<E> {
    pub fn new(endpoint: Endpoint, e: E) -> Self {
        EventFromEndpoint { event: e, endpoint }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub name: String,
    pub ent_id: NetEntId,
}
