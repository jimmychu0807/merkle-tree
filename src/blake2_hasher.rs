use blake2::{Blake2s256, Digest};

use crate::traits::{Hash, Hasher};

pub struct Blake2Hasher {}

impl Default for Blake2Hasher {
  fn default() -> Self {
    Self::new()
  }
}

impl Blake2Hasher {
  pub fn new() -> Self {
    Blake2Hasher {}
  }
}

impl Hasher for Blake2Hasher {
  fn hash<I: AsRef<[u8]>>(&self, input: I) -> Hash {
    Blake2s256::digest(input).to_vec()
  }

  fn hash_two<I: AsRef<[u8]>>(&self, input1: I, input2: I) -> Hash {
    Blake2s256::new().chain_update(input1).chain_update(input2).finalize().to_vec()
  }
}
