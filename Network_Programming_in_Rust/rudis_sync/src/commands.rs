use crate::RUDIS_DB;
use std::io::BufReader;
use std::net::TcpStream;

pub fn process_client_request(msg: &BufReader<TcpStream>) -> Vec<u8> {
    vec![8]
}
