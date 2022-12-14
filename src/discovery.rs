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

use crate::chat;

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

#[derive(Debug, PartialEq)]
pub struct Discovered {
    pub peer: chat::Peer,
}

#[derive(Debug)]
pub struct Config {
    pub host_info: Arc<HostInfo>,
    pub discovered_channel: Sender<Discovered>,
}

pub fn start(config: Config) {
    if !is_running() {
        println!("Starting discovery with config: {:?}", config);
        set_running(true);

        let receive_socket = UdpSocket::bind("0.0.0.0:42069").expect("Could not bind UDP socket");

        receive_socket.join_multicast_v4(&Ipv4Addr::from_str(MULTICAST_ADDRESS).unwrap(), &Ipv4Addr::UNSPECIFIED).expect("Could not join multicast group");
        receive_socket.set_multicast_loop_v4(false).unwrap();
        let send_socket = receive_socket.try_clone().unwrap();

        let mut threads = vec![];
        threads.push(thread::spawn(move || receive_discover(receive_socket, config.discovered_channel)));
        threads.push(thread::spawn(move || send_discover(config.host_info.clone(), send_socket)));

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

fn receive_discover(socket: UdpSocket, send_received: Sender<Discovered>) {
    while is_running() {
        let mut buffer = [0; 1024];
        match socket.peek_from(&mut buffer) {
            Ok((size, peer)) => {
                let mut buffer = vec![0; size];
                socket.recv_from(&mut buffer).expect("Failed to receive data");
                let peer_host_info: HostInfo = rmp_serde::from_slice(&buffer).unwrap();
                let discovered = Discovered {
                    peer: chat::Peer::new(Some(peer_host_info.id.clone()), &peer_host_info.name, peer)
                };
                send_received.send(discovered).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn send_discover(host_info: Arc<HostInfo>, socket: UdpSocket) {
    while is_running() {
        let host_info: &HostInfo = host_info.borrow();

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