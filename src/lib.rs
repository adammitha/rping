#![allow(dead_code)]
mod raw_socket;
mod icmp;

use std::io::Result;

use raw_socket::RawSocket;


pub struct RPing {
    socket: RawSocket,
    timeout: u32,
}

impl RPing {
    pub fn new(timeout: u32) -> Result<Self> {
        Ok(Self {
            socket: RawSocket::new()?,
            timeout,
        })
    }
}
