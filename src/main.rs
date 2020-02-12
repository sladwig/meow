/*
pbz-hash

Fast file hasher in Rust using MeowHash.
*/

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use digest::*;
use meowhash::*;

const BUF_SIZE: usize = 1024 * 1024;

fn main() {
    let args = env::args();

    // Show help screen and exit if file is not given
    if args.len() < 2 {
        println!("pbz-hash - Hash large files efficiently.");
        println!("    Usage: pbz-hash <filename>");
        std::process::exit(0);
    }

    let filename = args.skip(1).next().unwrap();

    // Handle files that do not exist
    if let Ok(filepath) = Path::new(&filename).canonicalize() {
        let mut file = File::open(filepath).unwrap();
        let mut buffer = [0; BUF_SIZE];
        let mut hasher = MeowHasher::new();
    
        // Update hash from file using buffer
        while file.read(&mut buffer[..]).unwrap() > 0 {
            hasher.input(&buffer[..]);
        }
    
        // Full 512-bit hash
        let result = hasher.result();
    
        // Truncate to 160-bits (like `git hash-object`)
        let hash = hex::encode(result).chars().take(40).collect::<String>();
    
        // Output hash to stdout
        println!("{}", &hash);
    
    // Let the user know when a file cannot be found
    } else {
        println!("Error: file not found: {}", filename);
    }
}
