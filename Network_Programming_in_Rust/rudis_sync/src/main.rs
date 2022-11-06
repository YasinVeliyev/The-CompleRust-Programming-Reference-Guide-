use lazy_static::lazy_static;
use std::io::{BufReader, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;
use std::{collections::HashMap, env};
mod commands;
use crate::commands::process_client_request;
type STORE = Mutex<HashMap<String, String>>;
lazy_static! {
    static ref RUDIS_DB: STORE = Mutex::new(HashMap::new());
}

fn main() {
    let addr = env::args()
        .skip(1)
        .next()
        .unwrap_or("127.0.0.1:6378".to_owned());
    let listener = TcpListener::bind(&addr).unwrap();
    println!("rudis_sync listening on {} ...", addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("New Connection from: {:?}", stream);
        let mut stream = BufReader::new(stream);
        let mut reply: Vec<u8> = Vec::new();
        stream.get_mut().write_all(&reply).unwrap();
        println!("{:?}", reply);
    }
}

fn handle_client(stream: TcpStream) {
    let mut stream = BufReader::new(stream);
    let reply = process_client_request(&stream);
    stream.get_mut().write_all(&reply).unwrap();
}
