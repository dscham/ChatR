#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod cli;
mod chat;
mod state;
mod discovery;

use std::borrow::{Borrow, BorrowMut};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use chrono;
use crate::discovery::HostInfo;

fn main() {
    tauri::Builder::default()
        .setup(|tauri_context| {
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_discovery,
            get_host_info,
            save_username,
            is_first_run,
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
    if is_first_run() {
        host_info = HostInfo::new(&username);
    } else {
        host_info = state::load_host_info().unwrap();
        host_info = HostInfo {
            id: host_info.id,
            name: username,
        };
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

#[tauri::command]
fn is_first_run() -> bool {
    match state::load_host_info() {
        Ok(_) => false,
        Err(_) => true,
    }
}