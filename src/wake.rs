//! The `wake` module implements the functionality for the _wake_ command 
//! 
//! # About
//! 
//! The goal is to broadcast a UDP magic Wake-on-LAN packet to a specified 
//! MAC address.
//! The MAC address, and any optional parameters, are specified in a config 
//! object, that the `main `function has obtained either from the command 
//! line arguments, or from a config file, or by some other means.
//! 

use crate::config::RunError;

use std::net::{IpAddr, SocketAddr, UdpSocket};




/// Broadcast a Wake-on-LAN magic `packet` on a UDP `port`
pub fn send_packet(packet: &[u8; 102], broadcast_addr: IpAddr, port: u16) -> Result<(), RunError> {
    let to_addr = SocketAddr::from((broadcast_addr, port));
    let from_addr = SocketAddr::from(([0, 0, 0, 0], 0));
    let socket = UdpSocket::bind(from_addr)?;
    socket.set_broadcast(true)?;
    socket.send_to(packet, to_addr)?;
    Ok(())
}

/// Create a Wake-on-LAN magic packet from `mac_address`
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Wake-on-LAN#Magic_packet) for details
pub fn make_packet(mac_address: &[u8; 6]) -> [u8; 102] {
    let mut packet = [0xFFu8; 102];
    let mut start = 6;
    for _ in 0..16 {
        packet[start..(start + 6)].copy_from_slice(mac_address);
        start += 6;
    }
    packet
}

