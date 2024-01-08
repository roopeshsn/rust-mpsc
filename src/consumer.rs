use std::sync::mpsc::Receiver;

use crate::types::Packet;

pub fn consumer(rx: Receiver<Packet>) {
    loop {
        println!("{}", rx.recv().unwrap())
    }
}
