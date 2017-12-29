use byteorder::{ByteOrder, LE};

pub struct Level<'a> {
    bytes: &'a [u8],
}

impl<'a> Level<'a> {
    pub fn new(bytes: &[u8]) -> Level {
        Level { bytes }
    }

    fn read_block(&self, position: usize) -> &[u8] {
        let start = LE::read_u32(&self.bytes[position..]) as usize;
        &self.bytes[start..start + LE::read_u32(&self.bytes[position + 4..]) as usize]
    }

    pub fn read_level_number(&self) -> u16 {
        LE::read_u16(self.read_block(0x40))
    }
}
