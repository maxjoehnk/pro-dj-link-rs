use crate::error::ProDjLinkResult;
use crate::packets::*;
use crate::{DeviceType, KeepAlivePackage, ANNOUNCEMENT_PORT, STATUS_PORT};
#[cfg(feature = "async-std")]
use async_std::net::UdpSocket;
use std::net::Ipv4Addr;
#[cfg(all(feature = "tokio", not(feature = "async-std")))]
use tokio::net::UdpSocket;

pub struct AsyncVirtualCdj {
    device: u8,
    socket: UdpSocket,
    buffer: [u8; 1024],
}

impl AsyncVirtualCdj {
    pub async fn new() -> ProDjLinkResult<Self> {
        Self::with_device(0x05).await
    }

    pub async fn with_device(device: u8) -> ProDjLinkResult<Self> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, STATUS_PORT)).await?;
        socket.set_multicast_ttl_v4(128)?;
        socket.set_broadcast(true)?;

        Ok(Self {
            device,
            socket,
            buffer: [0u8; 1024],
        })
    }

    pub async fn recv(&mut self) -> ProDjLinkResult<Option<StatusPacket>> {
        let bytes = self.socket.recv(&mut self.buffer).await?;

        let packet = StatusPacket::parse(&self.buffer[0..bytes]);

        if packet.is_none() {
            println!("ignoring invalid packet: {:?}", self.buffer);
        }

        Ok(packet)
    }

    pub async fn send_keep_alive(&self) -> ProDjLinkResult<()> {
        let packet = KeepAlivePackage {
            name: "Virtual CDJ".into(),
            ip: Ipv4Addr::new(192, 168, 1, 13),
            mac: [0xfc, 0x34, 0x97, 0xa2, 0x0b, 0xa6],
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
