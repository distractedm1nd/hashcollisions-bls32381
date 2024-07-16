use sha2::{Digest, Sha256};
use std::collections::HashSet;

const MODULUS: [u8; 32] = [
    0x73, 0xed, 0xa7, 0x53, 0x29, 0x9d, 0x7d, 0x48, 0x33, 0x39, 0xd8, 0x08, 0x09, 0xa1, 0xd8, 0x05,
    0x53, 0xbd, 0xa4, 0x02, 0xff, 0xfe, 0x5b, 0xfe, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01,
];
const NUM_VALUES: u64 = 100_000_00;

fn main() {
    let modulus = bytes_to_bigint(&MODULUS);
    let mut set: HashSet<Vec<u8>> = HashSet::new();
    let mut collisions = 0;

    for i in 0..NUM_VALUES {
        if i % 1_000_000 == 0 {
            println!("Hashed {} values", i);
            println!(
                "Collision probability: {:.6}",
                collisions as f64 / NUM_VALUES as f64
            );
        }
        let hash = hash_value(i);
        let modded_hash = mod_bigint(&hash, &modulus);
        if !set.insert(modded_hash) {
            collisions += 1;
        }
    }

    println!("Total values hashed: {}", NUM_VALUES);
    println!("Collisions found: {}", collisions);
    println!(
        "Collision probability: {:.6}",
        collisions as f64 / NUM_VALUES as f64
    );
}

fn hash_value(value: u64) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(value.to_le_bytes());
    hasher.finalize().to_vec()
}

fn bytes_to_bigint(bytes: &[u8]) -> num_bigint::BigUint {
    num_bigint::BigUint::from_bytes_be(bytes)
}

fn mod_bigint(hash: &[u8], modulus: &num_bigint::BigUint) -> Vec<u8> {
    let hash_bigint = num_bigint::BigUint::from_bytes_be(hash);
    let result = hash_bigint % modulus;
    result.to_bytes_be()
}
