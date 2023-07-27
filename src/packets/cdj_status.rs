use crate::packets::status::Status;

pub type CdjStatus<'a> = Status<'a, 0x0a, 0x92, 0x8c, 0x89>;
