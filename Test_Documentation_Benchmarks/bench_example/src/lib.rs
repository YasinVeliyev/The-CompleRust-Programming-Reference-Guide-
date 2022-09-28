#![feature(test)]
extern crate test;
use test::Bencher;

pub fn do_nothing_slowly() {
    print!(".");
    for i in 1..10_000_000 {
        print!("=")
    }
}

pub fn do_nothing_fast() {}

#[bench]
fn bench_nothing_slowly(b: &mut Bencher) {
    b.iter(|| do_nothing_slowly())
}

fn bench_nothing_fast(b: &mut Bencher) {
    b.iter(|| do_nothing_fast())
}
