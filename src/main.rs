/*
pbz-hash

Fast file hasher in Rust using MeowHash.
*/

//extern crate digest;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, Empty};
use std::io::prelude::*;
use digest::*;
use meowhash::*;
use std::io;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");

    let mut hasher = MeowHasher::new();

    //let data = vec![1, 2, 3, 4, 5];
    //hasher.input(&data);

    //println!("{:?}", result);
    //println!("{}", hex::encode(result));

    let file = OpenOptions::new().read(true).open(".gitignore").unwrap();
    let mut buf_reader = BufReader::new(file);


    let buf_size = 1024 * 1024;
    let mut buffer = Vec::with_capacity(buf_size);
    loop {
    	buf_reader.read(&mut buffer).unwrap();
    	hasher.input(&buffer);
    	if buf_reader.buffer().len() <= buf_size {
    		break;
    	}
	}
    let result = hasher.result();

    println!("{}", hex::encode(result));
}
