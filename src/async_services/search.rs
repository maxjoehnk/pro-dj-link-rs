use crate::constants::ANNOUNCEMENT_PORT;
use crate::error::ProDjLinkResult;
use crate::packets::KeepAlivePackage;
#[cfg(feature = "async-std")]
use async_std::net::UdpSocket;
use std::net::Ipv4Addr;
#[cfg(all(feature = "tokio", not(feature = "async-std")))]
use tokio::net::UdpSocket;

pub struct AsyncSearchService {
    socket: UdpSocket,
    buffer: [u8; 1024],
}

impl AsyncSearchService {
    pub async fn new() -> ProDjLinkResult<Self> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, ANNOUNCEMENT_PORT)).await?;

        socket.set_broadcast(true)?;
        socket.set_multicast_ttl_v4(128)?;

        Ok(Self {
            socket,
            buffer: [0u8; 1024],
        })
    }

    pub async fn recv(&mut self) -> ProDjLinkResult<Option<KeepAlivePackage>> {
        let buffer_size = self.socket.recv(&mut self.buffer).await?;

        let buf = &self.buffer[0..buffer_size];

        let package = KeepAlivePackage::try_parse(buf);

        Ok(package)
    }
}
