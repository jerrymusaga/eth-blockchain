use std::vec;

use hex;
use sha2::{Digest, Sha256};


pub fn hash_function(array: Vec<&str>){
    let mut sha256 = Sha256::new();
    for s in &array {
        sha256.update(s.as_bytes());
    }
    let result = sha256.finalize();
    println!("SHA256: 0x{}", hex::encode(result));

}

pub fn print() {
    let array = vec!["Blue", "Red", "White"];
    hash_function(array);
}