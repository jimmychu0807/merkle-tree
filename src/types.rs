// External dependencies
use core::fmt::{self, Debug};
use thiserror::Error;

// Internal dependencies
use crate::utils::hashes_to_str;

pub type Hash = Vec<u8>;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Empty leaf")]
  EmptyLeaf,
  #[error("Index out of bound")]
  IndexOutOfBound,
  #[error("Unknown error")]
  Unknown,
}

pub trait Hasher {
  fn hash<I: AsRef<[u8]>>(&self, input: I) -> Hash;

  fn hash_two<I: AsRef<[u8]>>(&self, input1: I, input2: I) -> Hash {
    let input1_ref = input1.as_ref();
    let input2_ref = input2.as_ref();

    let mut buf = vec![0; input1_ref.len() + input2_ref.len()];
    buf[..input1_ref.len()].clone_from_slice(input1_ref);
    buf[input1_ref.len()..].clone_from_slice(input2_ref);
    self.hash(&buf)
  }
}

pub struct MerkleProof<N> {
  pub hashes: Vec<Hash>,
  pub node_number: usize,
  pub index: usize,
  pub node: N,
}

impl<N: AsRef<[u8]> + Clone> Debug for MerkleProof<N> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("MerkleProof")
      .field("hashes", &hashes_to_str(&self.hashes))
      .field("node_number", &self.node_number)
      .field("index", &self.index)
      .finish()
  }
}

pub trait MerkleTree {
  type Hasher: Hasher;

  fn new(hasher: Self::Hasher) -> Self;
  fn merkle_root<N: AsRef<[u8]>>(&self, leaves: &[N]) -> Hash;
  fn merkle_proof<N: AsRef<[u8]> + Clone>(&self, leaves: &[N], index: usize) -> Result<MerkleProof<N>, Error>;
  fn verify_proof<N: AsRef<[u8]>>(&self, root: &Hash, proof: &MerkleProof<N>) -> bool;
}
