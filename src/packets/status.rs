use crate::buffer_ext::BufferExt;
use crate::{Speed, State};
use std::borrow::Cow;

const HEADER: [u8; 10] = [0x51, 0x73, 0x70, 0x74, 0x31, 0x57, 0x6d, 0x4a, 0x4f, 0x4c];

#[derive(Debug, Clone)]
pub struct Status<
    'a,
    const HEADER_LAST_BIT: u8,
    const BPM_ADDR: u8,
    const PITCH_ADDR: u8,
    const STATE_ADDR: u8,
> {
    pub name: Cow<'a, str>,
    pub device_id: u8,
    pub state: State,
    pub speed: Speed,
}

impl<
        'a,
        const HEADER_LAST_BIT: u8,
        const BPM_ADDR: u8,
        const PITCH_ADDR: u8,
        const STATE_ADDR: u8,
    > Status<'a, HEADER_LAST_BIT, BPM_ADDR, PITCH_ADDR, STATE_ADDR>
{
    pub fn try_parse(buffer: &'a [u8]) -> Option<Self> {
        let mut header = HEADER.to_vec();
        header.push(HEADER_LAST_BIT);
        if buffer.starts_with(&header) {
            let name = buffer.read_string(0x0b, 0x1e);
            let state = State::from_bits_truncate(buffer[0x89]);
            let speed = Speed {
                original: buffer.read_i16(0x92),
                pitch: buffer.read_i32(0x54),
            };

            Some(Self {
                name,
                device_id: buffer[0x21],
                state,
                speed,
            })
        } else {
            None
        }
    }
}
