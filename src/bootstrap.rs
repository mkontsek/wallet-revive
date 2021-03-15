use std::{env, time::Instant};

use global_counter::primitive::exact::CounterI32;
use rayon::prelude::*;
use secp256k1::{All, PublicKey, Secp256k1, SecretKey};

use crate::config::{AMOUNT_OF_TRIES, MOTHERLOAD_ADDRESS};
use crate::types::{OutputKey, RipeHash};
use crate::utils::{decode_to_ripehash, get_random_private_key, transmute_u8};
use std::sync::{Arc, Mutex};

static GLOBAL_COUNTER: CounterI32 = CounterI32::new(0);

fn get_ripehash() -> Box<RipeHash> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        decode_to_ripehash(&args[1])
    } else {
        decode_to_ripehash(&String::from(MOTHERLOAD_ADDRESS))
    }
}

pub fn find_private_key(
    source_public_key: &RipeHash,
    secp: &Secp256k1<All>,
    privkey: &OutputKey,
) -> Option<OutputKey> {
    GLOBAL_COUNTER.inc();

    let secret_key = SecretKey::from_slice(privkey).expect("32 bytes, within curve order");
    let newpubkey = PublicKey::from_secret_key(secp, &secret_key);
    let new_public_key_ripehash = transmute_u8::<PublicKey, RipeHash>(newpubkey);

    if new_public_key_ripehash == *source_public_key {
        let secret_key_u8 = transmute_u8::<SecretKey, OutputKey>(secret_key);
        println!("public key random:{:x?}", new_public_key_ripehash);
        println!("public key tried:{:x?}", *source_public_key);
        println!("Success:{:x?}", secret_key_u8);

        return Some(secret_key_u8);
    }

    None
}

pub fn run() {
    let ripehash = *get_ripehash();

    println!("Start private key search with {:} tries", AMOUNT_OF_TRIES);

    let now = Instant::now();
    let secp = Secp256k1::new();
    let outcome = Arc::new(Mutex::new(String::from(
        "No private key found.\nBetter luck next time.",
    )));

    (0..AMOUNT_OF_TRIES)
        .collect::<Vec<u32>>()
        .par_iter_mut()
        .for_each(|_| {
            let random_key = get_random_private_key();
            if let Some(_) = find_private_key(&ripehash, &secp, &random_key) {
                let mut outcome = outcome.lock().unwrap();
                *outcome = String::from("Congratulations! You have found the private key.")
            }
        });

    let final_outcome = outcome.lock().unwrap();
    println!(
        "\nTime taken for {:} tries: {:.3} seconds and {:} threads.\n{:}",
        AMOUNT_OF_TRIES,
        now.elapsed().as_nanos() as f64 / 1000_000_000.0,
        GLOBAL_COUNTER.get(),
        *final_outcome
    );
}
