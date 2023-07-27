use crate::packets::status::Status;

pub type MixerStatus<'a> = Status<'a, 0x29, 0x2e, 0x28, 0x27>;
