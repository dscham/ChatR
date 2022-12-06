use std::{thread, time};
use std::borrow::BorrowMut;
use std::net::{Ipv4Addr, SocketAddr, TcpStream, UdpSocket};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;

use serde::{Deserialize, Serialize};
use rmp_serde;

use chrono::Local;

static mut RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Peer {
    pub name: String,
    pub socket_addr: SocketAddr,
    pub last_seen: u64,
}

impl Peer {
    pub fn new(name: &str, socket_addr: SocketAddr, last_seen: u64) -> Peer {
        Peer {
            name: name.to_string(),
            socket_addr,
            last_seen,
        }
    }
}

pub type HostInfo = Peer;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Discovered {
    pub peer: Peer,
}

#[derive(Debug)]
pub struct Config {
    pub host_info: Arc<Mutex<HostInfo>>,
    pub discovered_channel: Sender<Discovered>,
}

pub fn start(config: Config) {
    if !is_running() {
        println!("Starting discovery with config: {:?}", config);
        set_running(true);

        let receive_socket = UdpSocket::bind("0.0.0.0:42069").expect("Could not bind UDP socket");
        receive_socket.join_multicast_v4(&Ipv4Addr::from_str("224.0.0.1").unwrap(), &Ipv4Addr::UNSPECIFIED).expect("Could not join multicast group");
        //receive_socket.set_multicast_loop_v4(false).unwrap();
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
                let peer: Peer = rmp_serde::from_slice(&buffer).unwrap();
                let discovered = Discovered { peer };
                println!("Received: {:?}", discovered);
                send_received.send(discovered).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn send_discover(host_info: Arc<Mutex<HostInfo>>, socket: UdpSocket) {
    while is_running() {
        let host_info = host_info.lock().unwrap();
        let host_info = HostInfo{
            name: host_info.name.clone(),
            socket_addr: host_info.socket_addr,
            last_seen: Local::now().timestamp().unsigned_abs(),
        };
        println!("Sending: {:?}", host_info);
        let serialized_info = rmp_serde::to_vec(&host_info).unwrap();
        println!("Sending Serialized: {:?}", serialized_info);
        socket.send_to(&serialized_info, "224.0.0.1:42069").expect("Failed to send data");
        thread::sleep(time::Duration::from_secs(10));
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