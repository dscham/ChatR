use std::{thread, time};
use std::borrow::{Borrow, BorrowMut};
use std::net::{Ipv4Addr, SocketAddr, TcpStream, UdpSocket};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;

use serde::{Deserialize, Serialize};
use rmp_serde;
use nanoid::nanoid;

use crate::{chat, state};

const MULTICAST_ADDRESS: &str = "224.0.0.1";
const MULTICAST_PORT: u16 = 42069;

static mut RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct HostInfo {
    pub id: String,
    pub name: String,
}

impl HostInfo {
    pub fn new(name: &str) -> HostInfo {
        HostInfo {
            id: nanoid::nanoid!(6),
            name: name.to_string(),
        }
    }
}

pub fn start() {
    if !is_running() {
        set_running(true);

        let receive_socket = UdpSocket::bind("0.0.0.0:42069").expect("Could not bind UDP socket");

        receive_socket.join_multicast_v4(&Ipv4Addr::from_str(MULTICAST_ADDRESS).unwrap(), &Ipv4Addr::UNSPECIFIED).expect("Could not join multicast group");
        //receive_socket.set_multicast_loop_v4(false).unwrap();
        let send_socket = receive_socket.try_clone().unwrap();

        let mut threads = vec![];
        threads.push(thread::spawn(move || receive_discover(receive_socket)));
        threads.push(thread::spawn(move || send_discover( send_socket)));

        for thread in threads {
            thread.join().unwrap();
        }
    }
}

pub fn stop() {
    if is_running() {
        set_running(false);
    }
}

fn receive_discover(socket: UdpSocket) {
    while is_running() {
        let mut buffer = [0; 1024];
        match socket.peek_from(&mut buffer) {
            Ok((size, peer_socket)) => {
                let mut buffer = vec![0; size];
                socket.recv_from(&mut buffer).expect("Failed to receive data");
                let peer: HostInfo = rmp_serde::from_slice(&buffer).unwrap();
                let peer = chat::Peer::new(Some(peer.id.clone()), &peer.name, peer_socket);

                let mut peers = state::load_peers().unwrap_or(vec![]);
                if peers.contains(&peer) {
                    peers.remove(peers.iter().position(|list_peer| list_peer == &peer).unwrap());
                }
                peers.push(peer);
                state::save_peers(&peers).unwrap();
            }
            Err(e) => {
            }
        }
    }
}

fn send_discover(socket: UdpSocket) {
    while is_running() {
        let host_info: HostInfo = state::load_host_info().unwrap();
        let serialized_info = rmp_serde::to_vec(&host_info).unwrap();
        socket.send_to(&serialized_info, format!("{}:{}", MULTICAST_ADDRESS, MULTICAST_PORT)).expect("Failed to send data");
        thread::sleep(time::Duration::from_secs(5));
    }
}

pub fn is_running() -> bool {
    unsafe {
        RUNNING.load(std::sync::atomic::Ordering::Relaxed)
    }
}

fn set_running(running: bool) {
    unsafe {
        RUNNING.store(running, std::sync::atomic::Ordering::Relaxed);
    }
}