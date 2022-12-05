use std::net::{SocketAddr, TcpStream};

pub static mut PEERS: Peers = Peers {
    peers: vec![],
};

pub struct Peer {
    pub name: String,
    pub connection: SocketAddr,
    pub last_seen: u64,
}

pub struct Peers {
    pub peers: Vec<Peer>,
}

impl Peer {
    pub fn new(name: &str, connection: SocketAddr, last_seen: u64) -> Peer {
        Peer {
            name: name.to_string(),
            connection,
            last_seen,
        }
    }

    fn compare(&self, other: &Peer) -> bool {
        self.name == other.name
            && self.connection.to_string() == other.connection.to_string()
    }
}

impl PartialEq for Peer {
    fn eq(&self, other: &Self) -> bool {
        self.compare(other)
    }
}

impl Peers {
    pub fn new() -> Peers {
        Peers { peers: Vec::new() }
    }

    pub fn add_peer(&mut self, peer: Peer) {
        if !self.peers.contains(&peer) {
            self.peers.push(peer);
        }
    }

    pub fn remove_peer(&mut self, peer: &Peer) {
        self.peers.retain(|p| !p.compare(peer));
    }

    pub fn get_peers(&self) -> &Vec<Peer> {
        &self.peers
    }
}