use std::thread;

fn alice() -> thread::JoinHandle<()> {
    thread::spawn(move || bob())
}

fn bob() {
    malice()
}

fn malice() {
    panic!("malice is panicking")
}

pub fn unwinding() {
    let child = alice();
    let _ = child.join();
    bob();
    println!("This is unreachable code");
}
