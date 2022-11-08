#![allow(dead_code)]
mod icmp;
mod raw_socket;

use std::io::Result;
use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::net::ToSocketAddrs;

use raw_socket::RawSocket;

pub struct RPing {
    socket: RawSocket,
    pub host: SocketAddrV4,
}

impl RPing {
    pub fn new(timeout: i64, host: impl ToSocketAddrs) -> Result<Self> {
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
}
