use sha3::{Digest, Keccak256};

use crate::traits::{Hash, Hasher};

pub struct Keccak256Hasher {}

impl Default for Keccak256Hasher {
  fn default() -> Self {
    Self::new()
  }
}

impl Keccak256Hasher {
  pub fn new() -> Self {
    Keccak256Hasher {}
  }
}

impl Hasher for Keccak256Hasher {
  fn hash<I: AsRef<[u8]>>(&self, input: I) -> Hash {
    Keccak256::digest(input).to_vec()
  }
}
