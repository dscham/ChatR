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
use crate::discovery::HostInfo;

mod cli;
mod chat;
mod state;
mod discovery;

fn main() {
    tauri::Builder::default()
        .setup(|tauri_context| {

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_discovery,
            get_host_info,
            save_username,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start_discovery() -> Result<(), String> {
    thread::spawn(|| {
        discovery::start();
    });
    Ok(())
}

#[tauri::command]
fn get_host_info() -> Result<HostInfo, String> {
    match state::load_host_info() {
        Ok(host_info) => {
           Ok(host_info)
        }
        Err(e) => {
            Err("Could not load host info".to_string())
        }
    }
}

#[tauri::command]
fn save_username(username: String) -> Result<HostInfo, String> {
    let mut host_info;
    match state::load_host_info() {
        Ok(loaded_host_info) => {
            host_info = loaded_host_info;
            host_info = HostInfo {
                id: host_info.id,
                name: username,
            };
        }
        Err(e) => {
            host_info = HostInfo::new(&username);
        }
    }
    match state::save_host_info(&host_info) {
        Ok(_) => {
            Ok(host_info)
        }
        Err(e) => {
            Err("Could not save username".to_string())
        }
    }
}