use hex::FromHex;
use std::fmt;

const MAC_ADDRESS_LENGTH: usize = 6;

#[derive(Debug)]
pub struct MacError {
    txt: String
}

impl fmt::Display for MacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.txt)
    }
}

pub struct Mac {
    bytes: Vec<u8>
}

impl Mac {
    pub fn as_bytes(self: Self) -> Vec<u8> {
        return self.bytes.clone();
    }
}

pub fn mac_from_str(value: &str) -> Result<Mac, MacError> {
    let mac_bytes = value.split(':')
        .flat_map(|v| Vec::from_hex(v).ok())
        .flatten()
        .collect::<Vec<u8>>();

    if mac_bytes.len() == MAC_ADDRESS_LENGTH {
        Ok(Mac{ bytes: mac_bytes })
    } else {
        Err(MacError{txt: String::from("Invalid MAC address")})
    }
}
