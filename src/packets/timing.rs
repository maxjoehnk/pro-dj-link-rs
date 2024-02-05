use std::borrow::Cow;

use crate::buffer_ext::BufferExt;

const HEADER: [u8; 11] = [
    0x51, 0x73, 0x70, 0x74, 0x31, 0x57, 0x6d, 0x4a, 0x4f, 0x4c, 0x28,
];

#[derive(Debug, Clone)]
pub struct TimingPacket<'a> {
    pub speed: Speed,
    pub device_id: u8,
    pub name: Cow<'a, str>,
    pub beat: u8,
}

impl<'a> TimingPacket<'a> {
    pub fn try_parse(buffer: &'a [u8]) -> Option<TimingPacket<'a>> {
        if buffer.starts_with(&HEADER) {
            let device = buffer[0x21];
            let speed = Speed {
                original: buffer.read_i16(0x5a),
                pitch: buffer.read_i32(0x54),
            };
            let name = buffer.read_string(0x0b, 0x1e);

            Some(TimingPacket {
                speed,
                device_id: device,
                name,
                beat: buffer[0x5c],
            })
        } else {
            tracing::debug!("ignoring invalid packet: {buffer:?}");
            None
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Speed {
    pub original: i16,
    pub pitch: i32,
}

impl Speed {
    pub fn current(&self) -> f64 {
        let current = self.original as f64 * self.pitch as f64;
        let current = current / 0x6400000 as f64;

        current
    }
}
