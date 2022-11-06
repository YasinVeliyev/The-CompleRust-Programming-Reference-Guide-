use std::sync::RwLock;
use std::thread;

pub fn thread_rwlock() {
    let m = RwLock::new(5);
    let c = thread::spawn(move || {
        *m.write().unwrap() += 1;
        let updated = *m.read().unwrap();
        updated
    });
    let updated = c.join().unwrap();
    println!("{:?}", updated);
}