use std::borrow::Cow;

pub trait BufferExt<'a> {
    fn read_string(&self, start: usize, end: usize) -> Cow<'a, str>;
    fn read_i32(&self, address: usize) -> i32;
    fn read_i16(&self, address: usize) -> i16;
}

impl<'a> BufferExt<'a> for &'a [u8] {
    fn read_string(&self, start: usize, end: usize) -> Cow<'a, str> {
        String::from_utf8_lossy(&self[start..end])
    }

    fn read_i32(&self, address: usize) -> i32 {
        i32::from_be_bytes([
            self[address],
            self[address + 1],
            self[address + 2],
            self[address + 3],
        ])
    }

    fn read_i16(&self, address: usize) -> i16 {
        i16::from_be_bytes([self[address], self[address + 1]])
    }
}
