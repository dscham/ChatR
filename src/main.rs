use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};

use chrono::Local;

mod util;
mod chat;
mod settings;
mod discovery;

fn main() {

    println!("Please enter a username: ");
    let username = util::read_cli_string();
    let peers = Arc::new(Mutex::new(chat::Peers::new()));
    let host_info = Arc::new(Mutex::new(discovery::HostInfo{
        name: username.clone(),
        socket_addr: SocketAddr::from_str("0.0.0.0:1234").unwrap(),
        last_seen: chrono::Local::now().timestamp().unsigned_abs(),
    }));

    let (send_discovered, recv_discovered): (Sender<discovery::Discovered>, Receiver<discovery::Discovered>) = mpsc::channel();

    discovery::start(discovery::Config{
        host_info: host_info.clone(),
        discovered_channel: send_discovered,
    });
}

enum Screen {
    HOME,
    PEERS,
    PEER,
    SETTINGS,
}