mod icmp;
mod raw_socket;
mod stats;

use std::collections::HashMap;
use std::fmt::Debug;
use std::net::{SocketAddr, SocketAddrV4, ToSocketAddrs};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use color_eyre::eyre::{eyre, Result, WrapErr};
use icmp::IcmpMessage;
use raw_socket::RawSocket;
use stats::Stats;

#[derive(Debug)]
pub struct RPing {
    socket: RawSocket,
    host: SocketAddrV4,
    cancelled: Arc<AtomicBool>,
    stats: Stats,
    timeout: u64,
}

impl RPing {
    pub fn new<T>(host: T, timeout: u64, cancelled: Arc<AtomicBool>) -> Result<Self>
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
            socket: RawSocket::new(&resolved_host)?,
            host: resolved_host,
            cancelled,
            stats: Stats::new(),
            timeout,
        })
    }

    pub fn start(&mut self, count: u16) -> Result<()> {
        println!("Pinging host {}", self.host.ip());
        let mut buf = [0u8; IcmpMessage::ICMP_HEADER_LEN];
        let mut req_map: HashMap<u16, IcmpMessage> = HashMap::new();

        for seq_num in 1..=count {
            if self.cancelled() {
                break;
            }
            // Construct packet
            let req = IcmpMessage::new_request(seq_num, None);
            req.serialize_packet(&mut buf)
                .wrap_err("Unable to serialize the ICMP message")?;
            req_map.insert(seq_num, req);

            // Send ICMP request
            self.socket.send(&buf)?;
            self.stats.send();

            // Wait for ICMP reply and report stats
            let mut time_remaining = Some(Duration::from_secs(self.timeout));
            while let Some(t) = time_remaining {
                let start = Instant::now();
                match self.socket.poll(t)  {
                    Ok(0) => break, // timeout
                    Ok(_) => {
                        let bytes_read = self.socket.recv(&mut buf)?;
                        let icmp_resp = IcmpMessage::deserialize_packet(&buf[..bytes_read as usize])?;
                        if let Some(req) = req_map.remove(&icmp_resp.seq_num()) {
                            let elapsed = req.timestamp().elapsed();
                            self.stats.recv(elapsed);
                            println!(
                                "Received {} bytes from {}: icmp_seq={}, time elapsed={3:.1}ms",
                                bytes_read,
                                self.host.ip(),
                                icmp_resp.seq_num(),
                                elapsed.as_secs_f64() * 1000.
                            );
                        }
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::Interrupted {
                            self.cancelled.store(true, Ordering::Relaxed);
                        }
                        break;
                    }
                }
                time_remaining = t.checked_sub(start.elapsed());
            }
            if let Some(_) = req_map.get(&seq_num) {
                println!("Timed out waiting for packet with icmp sequence number {}", seq_num);
            }
        }
        Ok(())
    }

    pub fn dump_stats(&self) {
        println!("\n--- {:?} stats ---", self.host.ip());
        print!("{}", self.stats);
    }

    fn cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}
