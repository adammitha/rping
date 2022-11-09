#![allow(dead_code)]
#![allow(unused_variables)]
mod icmp;
mod raw_socket;

use std::io::Result;
use std::net::{SocketAddr, SocketAddrV4, ToSocketAddrs};
// use std::time::{Duration, Instant};

use raw_socket::RawSocket;

pub struct RPing {
    socket: RawSocket,
    pub host: SocketAddrV4,
}

impl RPing {
    pub fn new(host: impl ToSocketAddrs, timeout: i64) -> Result<Self> {
        let resolved_host: SocketAddrV4 = match host
            .to_socket_addrs()?
            .filter(|a| matches!(a, SocketAddr::V4(_)))
            .next()
        {
            Some(SocketAddr::V4(addr)) => Ok(addr),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to resolve the host",
            )),
        }?;
        Ok(Self {
            socket: RawSocket::new(timeout, &resolved_host)?,
            host: resolved_host,
        })
    }

    pub fn start(&self, count: Option<u64>) {
        println!("pinging host: {} with count: {:?}", self.host.ip(), count);
        // for seq_num in 1..=count.unwrap_or(u64::MAX) {
        //     let start = Instant::now();
        //     // Send ICMP request and wait for reply
        //     if let Some(delay) = Duration::from_secs(1).checked_sub(start.elapsed()) {
        //         std::thread::sleep(delay);
        //     }
        // }
    }
}
