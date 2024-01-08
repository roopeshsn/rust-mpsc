use crate::types::{Packet, ProtocolType};
use std::net::Ipv4Addr;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub fn producer_one(tx: Sender<Packet>) {
    let mut sender_ip = Ipv4Addr::new(192, 168, 1, 1);
    loop {
        let sender_ip_str = sender_ip.to_string();
        let packet = Packet::new(
            &sender_ip_str,
            "192.168.2.1",
            vec![0, 1, 2, 3],
            ProtocolType::TCP,
        );
        tx.send(packet).expect("Failed to send!");
        sender_ip = increment_octet(sender_ip, 3);
        thread::sleep(Duration::from_secs(3));
    }
}

fn increment_octet(ip: Ipv4Addr, octet_index: usize) -> Ipv4Addr {
    let mut octets: [u8; 4] = ip.octets();
    octets[octet_index] = octets[octet_index].wrapping_add(1); // Increment the octet
    Ipv4Addr::from(octets)
}
