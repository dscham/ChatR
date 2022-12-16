use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{Write};
use crate::discovery::HostInfo;
use crate::chat::Peers;
use rmp_serde;

static HOST_INFO_FILE: &str = "host_info.chatrmp";
static PEERS_FILE: &str = "peers.chatrmp";


pub fn load_host_info() -> Result<HostInfo, rmp_serde::decode::Error> {
    let mut host_info = open_file(HOST_INFO_FILE);
    rmp_serde::from_read(host_info)
}

pub fn save_host_info(host_info: &HostInfo) -> Result<(), rmp_serde::encode::Error> {
    let mut file = open_file(HOST_INFO_FILE);
    let host_info = rmp_serde::to_vec(host_info);
    match host_info {
        Ok(host_info) => {
            Ok(write_file(&mut file, host_info.as_slice()).unwrap())
        }
        Err(err) => Err(err),
    }
}

pub fn load_peers() -> Result<Peers, rmp_serde::decode::Error> {
    let mut peers = open_file(PEERS_FILE);
    rmp_serde::from_read(peers)
}

pub fn save_peers(peers: &Peers) -> Result<(), rmp_serde::encode::Error> {
    let mut file = open_file(PEERS_FILE);
    let peers = rmp_serde::to_vec(peers);
    match peers {
        Ok(peers) => {
            Ok(write_file(&mut file, peers.as_slice()).unwrap())
        }
        Err(err) => Err(err),
    }

}

fn open_file(path: &str) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Could not open file")
}

fn write_file(file: &mut File, data: &[u8]) -> Result<(), std::io::Error> {
    match file.write_all(data) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}