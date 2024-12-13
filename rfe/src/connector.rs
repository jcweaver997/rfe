use core::fmt::Debug;

use crate::msg::MsgPacket;
extern crate alloc;
use alloc::vec::Vec;

pub trait Connector: Debug {
    fn send(&mut self, msgs: Vec<MsgPacket>);
    fn recv(&mut self) -> Option<Vec<MsgPacket>>;
}

#[cfg(feature = "std")]
mod connector_std {
    extern crate alloc;
    extern crate std;
    use alloc::vec::Vec;
    use anyhow::{anyhow, Result};
    use bincode::{decode_from_slice, encode_to_vec};
    use core::time::Duration;
    use log::*;
    use mio::net::{TcpListener, TcpStream, UdpSocket};
    use std::{
        io::{Read, Write},
        net::ToSocketAddrs,
        sync::mpsc::{self, Receiver, Sender},
    };

    use super::Connector;
    use crate::{msg::MsgPacket, BINCODE_CONFIG};

    #[derive(Debug)]
    pub struct MemConnector {
        sender: Sender<Vec<MsgPacket>>,
        receiver: Receiver<Vec<MsgPacket>>,
    }

    impl MemConnector {
        pub fn new() -> (Self, Self) {
            let (s1, r1) = mpsc::channel();
            let (s2, r2) = mpsc::channel();
            return (
                Self {
                    sender: s1,
                    receiver: r2,
                },
                Self {
                    sender: s2,
                    receiver: r1,
                },
            );
        }
    }

    impl Connector for MemConnector {
        fn send(&mut self, msgs: Vec<MsgPacket>) {
            self.sender.send(msgs).ok();
        }

        fn recv(&mut self) -> Option<Vec<MsgPacket>> {
            self.receiver.recv_timeout(Duration::ZERO).ok()
        }
    }

    #[derive(Debug)]
    pub struct TcpConnector {
        listener: TcpListener,
        listen: Option<TcpStream>,
        client: TcpStream,
        remote_addr: &'static str,
        remote_port: u16,
    }

    impl TcpConnector {
        pub fn new(
            local_addr: &'static str,
            local_port: u16,
            remote_addr: &'static str,
            remote_port: u16,
        ) -> Result<Self> {
            Ok(Self {
                listener: TcpListener::bind(
                    (local_addr, local_port)
                        .to_socket_addrs()?
                        .next()
                        .ok_or(anyhow!("failed to parse ip address"))?,
                )?,
                client: TcpStream::connect(
                    (remote_addr, remote_port)
                        .to_socket_addrs()?
                        .next()
                        .ok_or(anyhow!("failed to parse ip address"))?,
                )?,
                listen: None,
                remote_addr,
                remote_port,
            })
        }
    }

    impl Connector for TcpConnector {
        fn send(&mut self, msgs: Vec<MsgPacket>) {
            let r = encode_to_vec(&msgs, BINCODE_CONFIG).expect("failed to serialize tcp packet");
            if let Err(e) = self.client.write(&r) {
                warn!("tcp write error {e}");
                self.client = TcpStream::connect(
                    (self.remote_addr, self.remote_port)
                        .to_socket_addrs()
                        .unwrap()
                        .next()
                        .ok_or(anyhow!("failed to parse ip address"))
                        .unwrap(),
                )
                .unwrap();
            }
        }

        fn recv(&mut self) -> Option<Vec<MsgPacket>> {
            let mut read_buf = [0_u8; 4096];
            if let Ok((stream, sa)) = self.listener.accept() {
                info!("new connection from {}", sa);
                self.listen = Some(stream);
            }
            if let Some(l) = &mut self.listen {
                if let Ok(a) = l.read(&mut read_buf) {
                    if let Ok((r, _)) =
                        decode_from_slice::<Vec<MsgPacket>, _>(&read_buf[0..a], BINCODE_CONFIG)
                    {
                        return Some(r);
                    }
                }
            }

            return None;
        }
    }

    #[derive(Debug)]
    pub struct UdpConnector {
        socket: UdpSocket,
    }

    impl UdpConnector {
        pub fn new(
            local_addr: &'static str,
            local_port: u16,
            remote_addr: &'static str,
            remote_port: u16,
        ) -> Result<Self> {
            let socket = UdpSocket::bind(
                (local_addr, local_port)
                    .to_socket_addrs()?
                    .next()
                    .ok_or(anyhow!("failed to parse ip address"))?,
            )?;
            socket.connect(
                (remote_addr, remote_port)
                    .to_socket_addrs()?
                    .next()
                    .ok_or(anyhow!("failed to parse ip address"))?,
            )?;
            Ok(Self { socket })
        }
    }

    impl Connector for UdpConnector {
        fn send(&mut self, msgs: Vec<MsgPacket>) {
            let r = encode_to_vec(&msgs, BINCODE_CONFIG).expect("failed to serialize udp packet");
            self.socket.send(&r).ok();
        }

        fn recv(&mut self) -> Option<Vec<MsgPacket>> {
            let mut read_buf = [0_u8; 4096];
            if let Ok(a) = self.socket.recv(&mut read_buf) {
                if let Ok((r, _)) =
                    decode_from_slice::<Vec<MsgPacket>, _>(&read_buf[0..a], BINCODE_CONFIG)
                {
                    return Some(r);
                }
            }

            return None;
        }
    }
}

#[cfg(feature = "std")]
pub use connector_std::*;
