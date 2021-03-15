use std::mem;

use rand::Rng;
use sha2::{digest::generic_array::GenericArray, Digest, Sha256};

use crate::config::{KEY_LENGTH, PUBLIC_KEY_HASH_LENGTH};
use crate::types::{OutputKey, RipeHash};

fn check_hash(key: &[u8], checksum: &[u8]) {
    let mut result = GenericArray::default();

    for i in 0..2 {
        let mut hasher = Sha256::new();

        if i < 1 {
            hasher.update(key);
        } else {
            hasher.update(result)
        };

        result = hasher.finalize();
    }

    println!("Double hash:{:x?}", result);
    assert_eq!(&result[0..4], checksum)
}

pub fn get_random_private_key() -> OutputKey {
    let mut newkey: OutputKey = [u8::MIN; KEY_LENGTH];

    let mut rng = rand::thread_rng();

    for i in 0..KEY_LENGTH {
        newkey[i] = rng.gen::<u8>();
    }

    newkey
}

pub fn transmute_u8<T, V>(bad_key: T) -> V {
    unsafe { mem::transmute_copy::<T, V>(&bad_key) }
}

pub fn decode_to_ripehash(address: &String) -> Box<RipeHash> {
    let decoded = bs58::decode(address).into_vec().unwrap();

    let ripehash = &decoded[0..PUBLIC_KEY_HASH_LENGTH];
    let checksum = &decoded[PUBLIC_KEY_HASH_LENGTH..25];

    println!("Ripehash and checksum: {:x?} {:x?}", ripehash, checksum);

    check_hash(ripehash, checksum);

    let mut self_key: RipeHash = [0; PUBLIC_KEY_HASH_LENGTH];
    self_key.clone_from_slice(ripehash);

    Box::new(self_key)
}
