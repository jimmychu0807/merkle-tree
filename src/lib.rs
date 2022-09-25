use blake2::Digest;

#[cfg(test)]
mod tests;

type Hash = Vec<u8>;

pub struct MerkleTree<H: Digest> {
  hasher: H,
}

impl<H: Digest> MerkleTree<H> {
  pub fn new(hasher: H) -> Self {
    Self { hasher }
  }

  pub fn merkle_root<I: Iterator>(&self, leaves: I) -> Hash {
    b"abcde".to_vec()
  }
}
