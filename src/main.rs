/*
pbz-hash

Fast file hasher in Rust using MeowHash.
*/

use std::fs::OpenOptions;
use std::io::prelude::*;
use digest::*;
use meowhash::*;

const BUF_SIZE: usize = 1024 * 1024;

fn main() {
    let mut hasher = MeowHasher::new();
    let mut file = OpenOptions::new().read(true).open(".gitignore").unwrap();    
    let mut buffer = [0; BUF_SIZE];

    while file.read(&mut buffer[..]).unwrap() > 0 {
        hasher.input(&buffer[..]);
    }

    println!("{}", hex::encode(hasher.result()));
}
