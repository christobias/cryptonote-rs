mod account;
mod types;
use std::mem;

extern crate rand;
extern crate ed25519_dalek;
extern crate chrono;
extern crate leb128;
extern crate tiny_keccak;
extern crate rust_base58;

use rand::Rng;
use rand::rngs::OsRng;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;

fn main() {
    let mut csprng: OsRng = OsRng::new().unwrap();
    let keypair: Keypair = Keypair::generate(&mut csprng);

    // assert_eq!(32, std::mem::size_of::<types::hash_t>());
    // println!("{}", std::mem::size_of::<types::hash_t>());
    println!("{}", std::mem::size_of::<account::Account>());
    println!("Hello, world!");
    println!("{}", account::unix_timestamp());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
