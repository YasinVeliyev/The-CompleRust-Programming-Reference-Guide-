use std::sync::mpsc::sync_channel;
use std::thread;

pub fn sync_channels() {
    let (tx, rx) = sync_channel(1);
    let tx_clone = tx.clone();
    let _ = tx.send(0);
    thread::spawn(move || {
        let _ = tx.send(1);
    });

    thread::spawn(move || {
        let _ = tx_clone.send(2);
    });
    println!("Received {} via channel", rx.recv().unwrap());
    println!("Received {} via channel", rx.recv().unwrap());
    println!("Received {} via channel", rx.recv().unwrap());
    println!("Received {:?} via channel", rx.recv());
}