#![allow(dead_code)]
#![allow(unused_variables)]
mod icmp;
mod raw_socket;

use std::fmt::Debug;
use std::net::{SocketAddr, SocketAddrV4, ToSocketAddrs};
use std::time::{Duration, Instant};

use color_eyre::eyre::{Result, WrapErr};
use icmp::{IcmpMessage, IpDatagramSlice};
use raw_socket::RawSocket;
use tracing::{info, instrument};

#[derive(Debug)]
pub struct RPing {
    socket: RawSocket,
    pub host: SocketAddrV4,
}

impl RPing {
    #[instrument]
    pub fn new<T>(host: T, timeout: i64) -> Result<Self>
    where
        T: ToSocketAddrs + Debug,
    {
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

    #[instrument]
    pub fn start(&self, count: Option<u16>) -> Result<()> {
        info!("Pinging host {}", self.host.ip());
        let mut send_buf = [0u8; IcmpMessage::ICMP_HEADER_LEN];
        let mut recv_buf = [0u8; 1500];

        for seq_num in 1..=count.unwrap_or(u16::MAX) {
            // Construct packet
            let req = IcmpMessage::new_request(seq_num, None);
            req.serialize_packet(&mut send_buf)
                .wrap_err("Unable to serialize the ICMP message")?;

            // Send ICMP request and wait for reply
            let start = Instant::now();
            self.socket.send(&send_buf)?;

            // TODO: handle timeout better than this
            let bytes_read = self.socket.recv(&mut recv_buf)?;
            let elapsed = start.elapsed();
            let ip_resp = IpDatagramSlice::new(&recv_buf[..bytes_read as usize]);
            let icmp_resp = IcmpMessage::deserialize_packet(ip_resp.payload())?;
            println!(
                "Received {bytes_read} bytes from {}: icmp_seq={}, time elapsed={}ms",
                self.host.ip(),
                icmp_resp.seq_num,
                elapsed.as_millis()
            );

            if let Some(delay) = Duration::from_secs(1).checked_sub(elapsed) {
                std::thread::sleep(delay);
            }
        }
        Ok(())
    }
}
