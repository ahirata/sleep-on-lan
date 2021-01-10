use crate::mac::Mac;
pub use std::cmp::PartialEq;

const MAGIC_PACKET_LENGTH: usize = 102;

#[derive(PartialEq)]
pub struct MagicPacket {
    bytes: Vec<u8>
}

impl MagicPacket {
    pub fn as_bytes(self: &Self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn as_mut_slice<'a>(self: &'a mut Self) -> &'a mut[u8] {
        self.bytes.as_mut_slice()
    }

    pub fn clear(self: &mut Self) {
        for n in 0..self.bytes.len() {
            self.bytes[n] = 0;
        }
    }

    /// Create a MagicPacket with a backing Vec<u8> already resized to fit
    /// a MagicPacket like this instance
    pub fn create_blank(self: &Self) -> MagicPacket {
        let mut blank: Vec<u8> = Vec::with_capacity(MAGIC_PACKET_LENGTH);
        blank.resize(MAGIC_PACKET_LENGTH, 0);
        return MagicPacket { bytes: blank };
    }
}

impl From<Mac> for MagicPacket {
    fn from(mac: Mac) -> Self {
        let mut magic =  Vec::with_capacity(MAGIC_PACKET_LENGTH);

        for _ in 0..6 {
            magic.push(0xff);
        }

        let mut bytes = mac.as_bytes();
        bytes.reverse();
        for _ in 0..16 {
            magic.extend(bytes.iter());
        }

        return MagicPacket{bytes: magic};
    }
}
