use std::borrow::Cow;
use std::io::{Cursor, Write};
use std::net::Ipv4Addr;

use crate::buffer_ext::BufferExt;
use crate::{ProDjLinkResult, PROLINK_HEADER};

pub(crate) const HEADER: [u8; 12] = [
    0x51, 0x73, 0x70, 0x74, 0x31, 0x57, 0x6d, 0x4a, 0x4f, 0x4c, 0x06, 0x00,
];

#[derive(Debug, Clone)]
pub struct KeepAlivePackage<'a> {
    pub name: Cow<'a, str>,
    pub device_id: u8,
    pub mac: [u8; 6],
    // pub mac: Cow<'a, str>,
    pub ip: Ipv4Addr,
    pub device_type: DeviceType,
}

impl<'a> KeepAlivePackage<'a> {
    pub fn is_virtual_cdj(&self) -> bool {
        self.device_type == DeviceType::CDJ && self.name.contains("Virtual CDJ")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DeviceType {
    CDJ = 0x01,
    Mixer = 0x03,
    Rekordbox = 0x04,
}

impl TryFrom<u8> for DeviceType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == DeviceType::CDJ as u8 {
            Ok(DeviceType::CDJ)
        } else if value == DeviceType::Mixer as u8 {
            Ok(DeviceType::Mixer)
        } else if value == DeviceType::Rekordbox as u8 {
            Ok(DeviceType::Rekordbox)
        } else {
            Err(())
        }
    }
}

impl<'a> KeepAlivePackage<'a> {
    pub fn try_parse(buffer: &'a [u8]) -> Option<Self> {
        if buffer.starts_with(&HEADER) && buffer.len() == 0x36 {
            let name = buffer.read_string(0xb, 0x20);
            let mac = [
                buffer[0x26],
                buffer[0x27],
                buffer[0x28],
                buffer[0x29],
                buffer[0x2a],
                buffer[0x2b],
            ];
            let ip = buffer.read_i32(0x2c);
            let ip = Ipv4Addr::from(ip as u32);

            Some(Self {
                name,
                device_id: buffer[0x24],
                device_type: DeviceType::try_from(buffer[0x34]).ok()?,
                mac,
                ip,
            })
        } else {
            tracing::debug!("ignoring invalid packet: {buffer:?}");
            None
        }
    }

    pub fn get_buffer(&self) -> ProDjLinkResult<[u8; 0x36]> {
        let mut name_buffer = Cursor::new([0u8; 20]);
        name_buffer.write_all(self.name.as_bytes())?;
        let name_buffer = name_buffer.into_inner();
        let mut buffer = Cursor::new([0u8; 0x36]);
        let mac = [0xfc, 0x34, 0x97, 0xa2, 0x0b, 0xa6];
        let ip = [192, 168, 1, 13];
        buffer.write_all(&PROLINK_HEADER)?;
        buffer.write_all(&[0x06, 0x00])?;
        buffer.write_all(&name_buffer)?;
        buffer.write_all(&[0x01, 0x02, 0x00, 0x36])?;
        buffer.write_all(&[self.device_id])?;
        buffer.write_all(&[DeviceType::CDJ as u8])?;
        buffer.write_all(&mac)?;
        buffer.write_all(&ip)?;
        buffer.write_all(&[0x01, 0x00, 0x00, 0x00])?;
        buffer.write_all(&[DeviceType::CDJ as u8])?;
        buffer.write_all(&[0x00])?;

        Ok(buffer.into_inner())
    }
}
