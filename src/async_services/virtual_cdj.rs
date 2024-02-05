use std::net::Ipv4Addr;

#[cfg(feature = "async-std")]
use async_std::net::UdpSocket;
#[cfg(all(feature = "tokio", not(feature = "async-std")))]
use tokio::net::UdpSocket;

use crate::error::ProDjLinkResult;
use crate::packets::*;
use crate::{DeviceType, KeepAlivePackage, ANNOUNCEMENT_PORT, STATUS_PORT};

pub struct AsyncVirtualCdj {
    device: u8,
    socket: UdpSocket,
    buffer: [u8; 1024],
    cdj_ip: Ipv4Addr,
    cdj_mac: [u8; 6],
}

impl AsyncVirtualCdj {
    pub async fn new(cdj_ip: Ipv4Addr, cdj_mac: [u8; 6]) -> ProDjLinkResult<Self> {
        Self::with_device(cdj_ip, cdj_mac, 0x05).await
    }

    pub async fn with_device(
        cdj_ip: Ipv4Addr,
        cdj_mac: [u8; 6],
        device: u8,
    ) -> ProDjLinkResult<Self> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, STATUS_PORT)).await?;
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

    pub async fn recv(&mut self) -> ProDjLinkResult<Option<StatusPacket>> {
        let bytes = self.socket.recv(&mut self.buffer).await?;

        let packet = StatusPacket::parse(&self.buffer[0..bytes]);

        if packet.is_none() {
            tracing::debug!("ignoring invalid packet: {:?}", self.buffer);
        }

        Ok(packet)
    }

    pub async fn send_keep_alive(&self) -> ProDjLinkResult<()> {
        let packet = KeepAlivePackage {
            name: "Virtual CDJ".into(),
            ip: self.cdj_ip,
            mac: self.cdj_mac,
            device_type: DeviceType::CDJ,
            device_id: self.device,
        };

        self.socket
            .send_to(
                &packet.get_buffer()?,
                (Ipv4Addr::BROADCAST, ANNOUNCEMENT_PORT),
            )
            .await?;

        Ok(())
    }
}
