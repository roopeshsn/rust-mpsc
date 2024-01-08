use ::std::thread;
use std::sync::mpsc::channel;

pub mod consumer;
pub mod producer;
pub mod types;

fn main() {
    println!("Hello, world!");

    let (tx, rx) = channel::<types::Packet>();

    println!("Main started...");

    let producer_handle = thread::Builder::new()
        .name("producer".to_string())
        .spawn(|| {
            producer::producer_one(tx);
        });

    let consumer_handle = thread::Builder::new()
        .name("consumer".to_string())
        .spawn(|| {
            consumer::consumer(rx);
        });

    producer_handle
        .expect("producer thread panicked!")
        .join()
        .unwrap();
    consumer_handle
        .expect("consumer thread panicked!")
        .join()
        .unwrap();

    println!("Main finished!");
}
