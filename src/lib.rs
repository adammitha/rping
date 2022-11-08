#![allow(dead_code)]
#![allow(unused_variables)]
mod icmp;
mod raw_socket;

use std::io::Result;
use std::net::{SocketAddr, SocketAddrV4, ToSocketAddrs};

use raw_socket::RawSocket;

pub struct RPing {
    socket: RawSocket,
    pub host: SocketAddrV4,
}

impl RPing {
    pub fn new(host: impl ToSocketAddrs, timeout: i64) -> Result<Self> {
        let resolved_host: SocketAddrV4 = match host
            .to_socket_addrs()?
            .filter(|a| {
                matches!(a, SocketAddr::V4(_))
            })
            .next()
        {
            Some(SocketAddr::V4(addr)) => Ok(addr),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to resolve the host",
            )),
        }?;
        Ok(Self {
            socket: RawSocket::with_timeout(timeout)?,
            host: resolved_host,
        })
    }

    pub fn start(&self, count: Option<u64>) {
        for i in 0..count.unwrap_or(u64::MAX) {
            todo!("Send ICMP request and wait for response");
        }
    }
}
