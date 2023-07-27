use crate::constants::ANNOUNCEMENT_PORT;
use crate::error::ProDjLinkResult;
use crate::packets::KeepAlivePackage;
use std::net::{Ipv4Addr, UdpSocket};

pub struct SearchService {
    socket: UdpSocket,
    buffer: [u8; 1024],
}

impl SearchService {
    pub fn new() -> ProDjLinkResult<Self> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, ANNOUNCEMENT_PORT))?;

        socket.set_broadcast(true)?;
        socket.set_multicast_ttl_v4(128)?;

        Ok(Self {
            socket,
            buffer: [0u8; 1024],
        })
    }

    pub fn recv(&mut self) -> ProDjLinkResult<Option<KeepAlivePackage>> {
        let buffer_size = self.socket.recv(&mut self.buffer)?;

        let buf = &self.buffer[0..buffer_size];

        let package = KeepAlivePackage::try_parse(buf);

        Ok(package)
    }
}
