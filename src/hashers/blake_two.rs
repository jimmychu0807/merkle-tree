use blake2::{Blake2s256, Digest};

use crate::types::{Hash, Hasher};

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
}
