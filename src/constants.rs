use bitflags::bitflags;

pub const ANNOUNCEMENT_PORT: u16 = 50000;
pub const BEAT_PORT: u16 = 50001;
pub const STATUS_PORT: u16 = 50002;

pub const ANNOUNCE_INTERVAL: u16 = 1500;

pub const PROLINK_HEADER: [u8; 10] = [0x51, 0x73, 0x70, 0x74, 0x31, 0x57, 0x6d, 0x4a, 0x4f, 0x4c];

bitflags! {
    #[derive(Default)]
    pub struct State: u8 {
        const BPM_SYNC = 0x02;
        const ON_AIR = 0x08;
        const SYNCED = 0x10;
        const MASTER = 0x20;
        const PLAYING = 0x40;
    }
}
