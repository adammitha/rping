use std::fmt::{Display, Error, Formatter};
use std::time::{Duration, Instant};

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

    fn rtt_min(&self) -> f64 {
        self.rtts.iter().min().unwrap().as_secs_f64() * 1000.
    }

    fn rtt_max(&self) -> f64 {
        self.rtts.iter().max().unwrap().as_secs_f64() * 1000.
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
            "{} packets transmitted, {} packets received, {}% packet loss, time {:?}",
            self.num_sent,
            self.num_rcvd,
            self.packet_loss(),
            self.start.elapsed()
        )?;
        write!(
            f,
            "rtt min/avg/max = {0:.3}/{1:.3}/{2:.3} ms",
            self.rtt_min(),
            self.rtt_max(),
            self.rtt_mean()
        )
    }
}
