#![allow(dead_code)]
#![allow(unused_variables)]
mod icmp;
mod raw_socket;
mod stats;

use std::fmt::Debug;
use std::net::{SocketAddr, SocketAddrV4, ToSocketAddrs};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use color_eyre::eyre::{eyre, Result, WrapErr};
use icmp::IcmpMessage;
use raw_socket::RawSocket;
use stats::Stats;
use tracing::instrument;

#[derive(Debug)]
pub struct RPing {
    socket: RawSocket,
    host: SocketAddrV4,
    canceller: Receiver<()>,
    stats: Stats,
}

impl RPing {
    #[instrument]
    pub fn new<T>(host: T, timeout: i64, canceller: Receiver<()>) -> Result<Self>
    where
        T: ToSocketAddrs + Debug,
    {
        let resolved_host: SocketAddrV4 = match host
            .to_socket_addrs()?
            .filter(|a| matches!(a, SocketAddr::V4(_)))
            .next()
        {
            Some(SocketAddr::V4(addr)) => Ok(addr),
            _ => Err(eyre!("Unable to resolve the host")),
        }?;
        Ok(Self {
            socket: RawSocket::new(timeout, &resolved_host)?,
            host: resolved_host,
            canceller,
            stats: Stats::new(),
        })
    }

    #[instrument]
    pub fn start(&mut self, count: u16) -> Result<()> {
        println!("Pinging host {}", self.host.ip());
        let mut buf = [0u8; IcmpMessage::ICMP_HEADER_LEN];

        for seq_num in 1..=count {
            // Construct packet
            let req = IcmpMessage::new_request(seq_num, None);
            req.serialize_packet(&mut buf)
                .wrap_err("Unable to serialize the ICMP message")?;

            // Send ICMP request
            let start = Instant::now();
            self.socket.send(&buf)?;
            self.stats.send();

            // Wait for ICMP reply and report stats
            let bytes_read = match self.socket.recv(&mut buf) {
                Ok(res) => Ok(res),
                Err(err) => match err.kind() {
                    std::io::ErrorKind::WouldBlock => {
                        println!("Timeout waiting for packet with seq_num {}", seq_num);
                        continue;
                    },
                    _ => Err(err),
                },
            }?;
            let elapsed = start.elapsed();
            let icmp_resp = IcmpMessage::deserialize_packet(&buf[..bytes_read as usize])?;
            println!(
                "Received {bytes_read} bytes from {}: icmp_seq={}, time elapsed={2:.1}ms",
                self.host.ip(),
                icmp_resp.seq_num,
                elapsed.as_secs_f64() * 1000.
            );
            self.stats.recv(elapsed);

            // Sleep for remainder of interval between sending packets
            if seq_num < count {
                let delay = Duration::from_secs(1).checked_sub(elapsed).unwrap_or(Duration::from_secs(0));
                if let Ok(_) = self.canceller.recv_timeout(delay) {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn dump_stats(&self) {
        println!("--- {:?} stats ---", self.host.ip());
        println!("{}", self.stats);
    }
}
