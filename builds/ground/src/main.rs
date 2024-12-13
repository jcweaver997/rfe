#![feature(thread_sleep_until)]

use std::{
    thread::sleep_until,
    time::{Duration, Instant},
};

use anyhow::Result;
use connector::{Connector, UdpConnector};
use log::*;
use rfe::*;
use simple_logger::SimpleLogger;

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    let mut udp = UdpConnector::new("127.0.0.1", 7011, "127.0.0.1", 7010)?;

    let mut next_time = Instant::now() + Duration::from_millis(10);
    loop {
        sleep_until(next_time);

        while let Some(msgs) = udp.recv() {
            for msg in msgs {
                info!("got msg {:?}", msg);
            }
        }
        next_time += Duration::from_millis(10);
    }
}
