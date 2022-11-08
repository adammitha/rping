#![allow(dead_code)]
mod raw_socket;
mod icmp;

use std::io::Result;

use raw_socket::RawSocket;


pub struct RPing {
    socket: RawSocket,
}

impl RPing {
    pub fn new(timeout: i64) -> Result<Self> {
        Ok(Self {
            socket: RawSocket::with_timeout(timeout)?,
        })
    }
}
