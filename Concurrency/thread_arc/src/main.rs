use std::sync::Arc;
use std::thread;

use arc_mutex::*;
use async_channels::*;
use mutex::*;
use sync_channels::*;
use thread_rwlock::*;

mod arc_mutex;
mod mutex;
mod thread_rwlock;
mod async_channels;
mod sync_channels;

fn main() {
    let nums = Arc::new(vec![0, 1, 2, 3, 4]);
    let mut childs = vec![];
    for n in 0..5 {
        let ns = Arc::clone(&nums);
        childs.push(thread::spawn(move || {
            println!("{}", ns[n]);
        }));
    }
    for c in childs {
        c.join();
    }
    mutex();
    arc_mutex();
    thread_rwlock();
    async_channels().join().unwrap();
    sync_channels();
}
