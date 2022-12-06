use std::io;
use std::net::{IpAddr, SocketAddr};

pub fn create_socket_addr(host: &str, port: u16) -> SocketAddr {
    let ip: IpAddr = host.parse().unwrap();
    SocketAddr::new(ip, port)
}

pub fn read_cli_u16() -> u16 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

pub fn read_cli_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}