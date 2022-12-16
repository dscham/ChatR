#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::borrow::{Borrow, BorrowMut};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use chrono;

mod cli;
mod chat;
mod state;
mod discovery;

struct Main {
}

impl Main {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        let (send_discovered, recv_discovered): (Sender<discovery::Discovered>, Receiver<discovery::Discovered>) = mpsc::channel();
        let handle_discovered_thread = thread::spawn(move || Self::handle_discovered(recv_discovered));

        discovery::start(discovery::Config {
            discovered_channel: send_discovered,
        });

        tauri::Builder::default()
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }

    fn handle_discovered(rx: Receiver<discovery::Discovered>) {
        loop {
            match rx.recv() {
                Ok(discovered) => {
                    let mut peers = state::load_peers().unwrap();
                    if peers.contains(&discovered.peer) {
                        continue;
                    }
                    peers.push(discovered.peer);
                    state::save_peers(&peers).unwrap();
                }
                Err(_) => {
                    break;
                }
            };
        }
    }
}

fn main() {
    Main::new().run();
}



