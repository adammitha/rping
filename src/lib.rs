#![allow(dead_code)]
#![allow(unused_variables)]
mod icmp;
mod raw_socket;

use std::fmt::{Debug, Display, Error, Formatter};
use std::net::{SocketAddr, SocketAddrV4, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use color_eyre::eyre::{Result, WrapErr};
use icmp::IcmpMessage;
use raw_socket::RawSocket;
use tracing::{info, instrument};

#[derive(Debug)]
pub struct RPing {
    socket: RawSocket,
    host: SocketAddrV4,
    cancelled: Arc<AtomicBool>,
    stats: Stats,
}

impl RPing {
    #[instrument]
    pub fn new<T>(host: T, timeout: i64, cancelled: Arc<AtomicBool>) -> Result<Self>
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
            cancelled,
            stats: Stats::new(),
        })
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    #[instrument]
    pub fn start(&mut self, count: Option<u16>) -> Result<()> {
        info!("Pinging host {}", self.host.ip());
        let mut buf = [0u8; IcmpMessage::ICMP_HEADER_LEN];

        for seq_num in 1..=count.unwrap_or(u16::MAX) {
            // Check for cancellation
            if self.is_cancelled() {
                break;
            }

            // Construct packet
            let req = IcmpMessage::new_request(seq_num, None);
            req.serialize_packet(&mut buf)
                .wrap_err("Unable to serialize the ICMP message")?;

            // Send ICMP request and wait for reply
            let start = Instant::now();
            self.socket.send(&buf)?;
            self.stats.send();

            // TODO: handle timeout better than this
            let bytes_read = self.socket.recv(&mut buf)?;
            let elapsed = start.elapsed();
            let icmp_resp = IcmpMessage::deserialize_packet(&buf[..bytes_read as usize])?;
            println!(
                "Received {bytes_read} bytes from {}: icmp_seq={}, time elapsed={}ms",
                self.host.ip(),
                icmp_resp.seq_num,
                elapsed.as_millis()
            );
            self.stats.recv(elapsed);

            if !self.is_cancelled() {
                if let Some(delay) = Duration::from_secs(1).checked_sub(elapsed) {
                    std::thread::sleep(delay);
                }
            }
        }
        Ok(())
    }

    pub fn dump_stats(&self) {
        println!("------------------------------------------------");
        println!("{}", self.stats);
    }
}

#[derive(Debug)]
pub struct Stats {
    rtts: Vec<Duration>,
    num_sent: usize,
    num_rcvd: usize,
    start: Instant,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            rtts: Vec::new(),
            num_sent: 0,
            num_rcvd: 0,
            start: Instant::now(),
        }
    }

    pub fn send(&mut self) {
        self.num_sent += 1;
    }

    pub fn recv(&mut self, rtt: Duration) {
        self.num_rcvd += 1;
        self.rtts.push(rtt);
    }

    fn packet_loss(&self) -> f64 {
        (1. - self.num_rcvd as f64 / self.num_sent as f64) * 100.
    }

    fn rtt_min(&self) -> u128 {
        self.rtts.iter().min().unwrap().as_millis()
    }

    fn rtt_max(&self) -> u128 {
        self.rtts.iter().max().unwrap().as_millis()
    }

    fn rtt_mean(&self) -> f64 {
        let mean = self.rtts.iter().sum::<Duration>() / self.rtts.len().try_into().unwrap();
        Into::<f64>::into(TryInto::<u32>::try_into(mean.as_micros()).unwrap()) / 1000.
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(
            f,
            "{} packets transmitted, {} packets received, {}% packet loss, time {:?}ms",
            self.num_sent,
            self.num_rcvd,
            self.packet_loss(),
            self.start.elapsed()
        )?;
        write!(
            f,
            "rtt min/avg/max = {}/{}/{} ms",
            self.rtt_min(),
            self.rtt_max(),
            self.rtt_mean()
        )
    }
}
