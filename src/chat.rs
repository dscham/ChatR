use std::net;
use nanoid::nanoid;
use chrono;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Peer {
    pub id: String,
    pub name: String,
    pub socket_addr: net::SocketAddr,
    pub last_seen: u64,
}

impl PartialEq for Peer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
        && self.socket_addr == other.socket_addr
    }
}

impl Peer {
    pub fn new(id: Option<String>, name: &str, socket_addr: net::SocketAddr) -> Peer {
        Peer {
            id: id.unwrap_or_else(|| nanoid!(6)),
            name: name.to_string(),
            socket_addr,
            last_seen: chrono::Local::now().timestamp().unsigned_abs(),
        }
    }
}

pub type Peers = Vec<Peer>;