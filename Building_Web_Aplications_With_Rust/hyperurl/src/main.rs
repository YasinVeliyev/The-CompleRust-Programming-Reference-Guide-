// use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::Server;
use log::{error, info};
use std::env;

mod service;
mod shortener;
fn main() {
    println!("Hello, world!");
}
