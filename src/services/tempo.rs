use crate::error::ProDjLinkResult;
use crate::packets::TimingPacket;
use crate::BEAT_PORT;
use std::net::{Ipv4Addr, UdpSocket};

pub struct TrackBpmService {
    socket: UdpSocket,
    buffer: [u8; 1024],
}

impl TrackBpmService {
    pub fn new() -> ProDjLinkResult<Self> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, BEAT_PORT))?;

        socket.set_broadcast(true)?;
        socket.set_multicast_ttl_v4(128)?;

        Ok(Self {
            socket,
            buffer: [0u8; 1024],
        })
    }

    pub fn recv(&mut self) -> ProDjLinkResult<Option<TimingPacket>> {
        let buffer_size = self.socket.recv(&mut self.buffer)?;

        let buf = &self.buffer[0..buffer_size];

        let package = TimingPacket::try_parse(buf);

        Ok(package)
    }
}
