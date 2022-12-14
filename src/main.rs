use std::borrow::{Borrow, BorrowMut};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use chrono;

mod cli;
mod chat;
mod settings;
mod discovery;

fn main() {
    println!("Please enter a username: ");
    let username = cli::read_string();
    let peers = Arc::new(Mutex::new(chat::Peers::new()));
    let host_info = Arc::new(discovery::HostInfo::new(&username));

    let (send_discovered, recv_discovered): (Sender<discovery::Discovered>, Receiver<discovery::Discovered>) = mpsc::channel();
    let handle_discovered_thread = thread::spawn(move || handle_discovered(recv_discovered, peers.clone()));

    discovery::start(discovery::Config{
        host_info: host_info.clone(),
        discovered_channel: send_discovered,
    });
}

fn handle_discovered(rx: Receiver<discovery::Discovered>, peers: Arc<Mutex<chat::Peers>>) {
    loop {
        match rx.recv() {
            Ok(discovered) => {
                let mut peers = peers.lock().unwrap();
                if peers.borrow().contains(&discovered.peer) {
                    continue;
                }
                peers.borrow_mut().push(discovered.peer);
                println!("Discovered peer! Peers:{:?}", peers.borrow());
            },
            Err(_) => {
                println!("Error receiving discovered peer");
                break
            },
        };
    }
}