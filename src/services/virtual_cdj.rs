use std::net::{Ipv4Addr, UdpSocket};

use crate::error::ProDjLinkResult;
use crate::{CdjStatus, DeviceType, KeepAlivePackage, MixerStatus, ANNOUNCEMENT_PORT, STATUS_PORT};

pub struct VirtualCdj {
    device: u8,
    socket: UdpSocket,
    buffer: [u8; 1024],
    cdj_ip: Ipv4Addr,
    cdj_mac: [u8; 6],
}

#[derive(Debug, Clone)]
pub enum Packet<'a> {
    CdjStatus(CdjStatus<'a>),
    MixerStatus(MixerStatus<'a>),
}

impl VirtualCdj {
    pub fn new(cdj_ip: Ipv4Addr, cdj_mac: [u8; 6]) -> ProDjLinkResult<Self> {
        Self::with_device(cdj_ip, cdj_mac, 0x05)
    }

    pub fn with_device(cdj_ip: Ipv4Addr, cdj_mac: [u8; 6], device: u8) -> ProDjLinkResult<Self> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, STATUS_PORT))?;
        socket.set_multicast_ttl_v4(128)?;
        socket.set_broadcast(true)?;

        Ok(Self {
            device,
            socket,
            buffer: [0u8; 1024],
            cdj_ip,
            cdj_mac,
        })
    }

    pub fn recv(&mut self) -> ProDjLinkResult<Option<Packet>> {
        let bytes = self.socket.recv(&mut self.buffer)?;

        let packet = CdjStatus::try_parse(&self.buffer[0..bytes])
            .map(Packet::CdjStatus)
            .or_else(|| MixerStatus::try_parse(&self.buffer[0..bytes]).map(Packet::MixerStatus));

        if packet.is_none() {
            tracing::debug!("ignoring invalid packet: {:?}", self.buffer);
        }

        Ok(packet)
    }

    pub fn send_keep_alive(&self) -> ProDjLinkResult<()> {
        let packet = KeepAlivePackage {
            name: "Virtual CDJ".into(),
            ip: self.cdj_ip,
            mac: self.cdj_mac,
            device_type: DeviceType::CDJ,
            device_id: self.device,
        };

        self.socket.send_to(
            &packet.get_buffer()?,
            (Ipv4Addr::BROADCAST, ANNOUNCEMENT_PORT),
        )?;

        Ok(())
    }
}
