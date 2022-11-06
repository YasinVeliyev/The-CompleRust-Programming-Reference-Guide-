use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;

pub fn async_channels() -> JoinHandle<()> {
    let (tx, rx) = channel();
    let join_handle = thread::spawn(move || {
        while let Ok(n) = rx.recv() {
            println!("Received {}", n);
        }
    });
    for i in 0..10 {
        tx.send(i).unwrap();
    }
    tx.send(0);
    join_handle
}