use std::net::{Ipv4Addr, TcpStream, UdpSocket};
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::{thread, time};
use chrono::Local;
use crate::peers;
use crate::peers::Peer;

pub struct Discovery {
    username: String,
    initialized: bool,
}

static mut RUNNING: AtomicBool = AtomicBool::new(false);

impl Discovery {
    pub fn new(username: &str) -> Self {
        Discovery {
            username: username.to_string(),
            initialized: true,
        }
    }

    pub fn start(&mut self) {
        let running = unsafe { RUNNING.load(std::sync::atomic::Ordering::Relaxed) };

        if !running & self.initialized {
            self.set_running(true);

            let username = self.username.clone();
            let socket = UdpSocket::bind("0.0.0.0:42069").expect("Could not bind UDP socket");
            socket.join_multicast_v4(&Ipv4Addr::from_str("224.0.0.1").unwrap(), &Ipv4Addr::UNSPECIFIED).expect("Could not join multicast group");
            socket.set_multicast_loop_v4(false).unwrap();
            let socket_copy = socket.try_clone().unwrap();

            thread::spawn(move || Self::receive_discover(socket));
            thread::spawn(move || Self::send_discover(username.as_str(), socket_copy));
        }
    }

    pub fn stop(&mut self) {
        let running = unsafe { RUNNING.load(std::sync::atomic::Ordering::Relaxed) };
        if running {
            self.set_running(false);
        }
    }

    pub fn is_running(&self) -> bool {
        unsafe { RUNNING.load(std::sync::atomic::Ordering::Relaxed) }
    }

    fn receive_discover(socket: UdpSocket) {
        while unsafe { RUNNING.load(std::sync::atomic::Ordering::Relaxed) } {
            let mut buffer = [0; 1024];
            match socket.peek_from(&mut buffer) {
                Ok((size, peer)) => {
                    let mut buffer = vec![0; size];
                    socket.recv_from(&mut buffer).expect("Failed to receive data");
                    unsafe {
                        let peer = Peer::new(
                            String::from_utf8_lossy(&buffer).trim(),
                            peer,
                            Local::now().timestamp().unsigned_abs(),
                        );

                        peers::PEERS.remove_peer(&peer);

                        peers::PEERS.add_peer(peer);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }

    fn send_discover(username: &str, socket: UdpSocket) {
        while unsafe { RUNNING.load(std::sync::atomic::Ordering::Relaxed) } {
            socket.send_to(username.as_bytes(), "224.0.0.1:42069").expect("Failed to send data");
            thread::sleep(time::Duration::from_secs(10));
        }
    }

    fn set_running(&mut self, running: bool) {
        unsafe {
            RUNNING.store(running, std::sync::atomic::Ordering::Relaxed);
        }
    }
}