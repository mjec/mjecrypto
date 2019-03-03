extern crate hex;

use mjecrypto::hash::sha256::hash;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        println!("Unable to read from stdin: {:?}", e);
        ::std::process::exit(1);
    }
    println!("{}  -", hex::encode(hash(buffer.as_bytes())));
}
