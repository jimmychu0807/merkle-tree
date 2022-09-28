use crate::{
  hashers::BlakeTwo256Hasher,
  types::{Error, Hash, Hasher, MerkleProof, MerkleTree as MerkleTreeT},
};

pub struct MerkleTreeRecursion<H> {
  hasher: H,
}

impl<H: Hasher> MerkleTreeT for MerkleTreeRecursion<H> {
  type Hasher = H;

  fn new(hasher: H) -> Self {
    Self { hasher }
  }

  fn merkle_root<N: AsRef<[u8]>>(&self, leaves: &[N]) -> Hash {
    // TODO
    b"something".to_vec()
  }

  fn merkle_proof<N: AsRef<[u8]> + Clone>(&self, leaves: &[N], index: usize) -> Result<MerkleProof<N>, Error> {
    // TODO
    Err(Error::Unknown)
  }

  fn verify_proof<N: AsRef<[u8]>>(&self, root: &Hash, proof: &MerkleProof<N>) -> bool {
    // TODO
    false
  }
}

impl Default for MerkleTreeRecursion<BlakeTwo256Hasher> {
  fn default() -> Self {
    Self::new(BlakeTwo256Hasher::default())
  }
}
