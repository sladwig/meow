/*
pbz-hash

Fast file hasher in Rust using MeowHash.
*/

//extern crate digest;

use digest::*;
use meowhash::*;

fn main() {
    println!("Hello, world!");

    let hasher = MeowHasher::new();

    //hasher.digest(3);

    hasher.input(3);
}
