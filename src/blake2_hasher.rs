use blake2::{Blake2s256, Digest};

use crate::traits::{Hash, Hasher};

pub struct BlakeTwo256Hasher {}

impl Default for BlakeTwo256Hasher {
  fn default() -> Self {
    Self::new()
  }
}

impl BlakeTwo256Hasher {
  pub fn new() -> Self {
    BlakeTwo256Hasher {}
  }
}

impl Hasher for BlakeTwo256Hasher {
  fn hash<I: AsRef<[u8]>>(&self, input: I) -> Hash {
    Blake2s256::digest(input).to_vec()
  }

  fn hash_two<I: AsRef<[u8]>>(&self, input1: I, input2: I) -> Hash {
    Blake2s256::new().chain_update(input1).chain_update(input2).finalize().to_vec()
  }
}
