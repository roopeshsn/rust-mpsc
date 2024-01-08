use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Packet {
    source_address: String,
    destination_address: String,
    payload: Vec<u8>,
    protocol: ProtocolType,
}

#[derive(Debug)]
pub enum ProtocolType {
    TCP,
    UDP,
}

impl Packet {
    pub fn new(
        source_address: &str,
        destination_address: &str,
        payload: Vec<u8>,
        protocol: ProtocolType,
    ) -> Self {
        Packet {
            source_address: source_address.to_string(),
            destination_address: destination_address.to_string(),
            payload,
            protocol,
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {:?}, {:?}",
            self.source_address, self.destination_address, self.payload, self.protocol
        )
    }
}
