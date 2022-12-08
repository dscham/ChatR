use std::net;

#[derive(Debug, PartialEq)]
pub struct Peer {
    pub name: String,
    pub socket_addr: net::SocketAddr,
    pub last_seen: u64,
}

impl Peer {
    pub fn new(name: &str, socket_addr: net::SocketAddr, last_seen: u64) -> Peer {
        Peer {
            name: name.to_string(),
            socket_addr,
            last_seen,
        }
    }
}

pub type Peers = Vec<Peer>;