mod bootstrap;
mod config;
mod types;
mod utils;

fn main() {
    bootstrap::run();
}

#[cfg(test)]
mod tests {
    extern crate hex;
    use super::*;
    use crate::types::{OutputKey, RipeHash};
    use secp256k1::{All, Secp256k1};
    use std::convert::TryInto;

    const PRIVATE_KEY: &str = "18e14a7b6a307f426a94f8114701e7c8e774e7f9a47e2c2035db29a206321725";
    const WRONG_PRIVATE_KEY: &str =
        "18e14a7b6a307f426a94f8114701e7c8e774e7f9a47e2c2035db29a206321724";
    const BTC_ADDRESS: &str = "1PMycacnJaSqwwJqjawXBErnLsZ7RkXUAs";

    fn get_args(private_key_str: &str) -> (Box<RipeHash>, Secp256k1<All>, OutputKey) {
        let secp = Secp256k1::new();
        let ripehash = utils::decode_to_ripehash(&String::from(BTC_ADDRESS));
        let decoded_private_key = hex::decode(private_key_str).expect("Decoding failed");
        let private_key: OutputKey = decoded_private_key.try_into().unwrap();

        (ripehash, secp, private_key)
    }

    #[test]
    fn find_key() {
        let (ripehash, secp, private_key) = get_args(PRIVATE_KEY);
        if let Some(found_key) = bootstrap::find_private_key(&ripehash, &secp, &private_key) {
            assert_eq!(found_key, private_key);
        }
    }

    #[test]
    fn do_not_find_key() {
        let (ripehash, secp, private_key) = get_args(WRONG_PRIVATE_KEY);
        if let Some(found_key) = bootstrap::find_private_key(&ripehash, &secp, &private_key) {
            assert_ne!(found_key, private_key);
        }
    }
}
