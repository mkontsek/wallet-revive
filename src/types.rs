use super::config::{KEY_LENGTH, PUBLIC_KEY_HASH_LENGTH};

pub type RipeHash = [u8; PUBLIC_KEY_HASH_LENGTH];
pub type OutputKey = [u8; KEY_LENGTH];
