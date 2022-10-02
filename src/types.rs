// External dependencies
use core::fmt::{self, Debug};
use thiserror::Error;

// Internal dependencies
use crate::utils::{hash_to_str, hashes_to_str};

pub type Hash = Vec<u8>;

#[derive(Error, Debug, PartialEq)]
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

#[derive(PartialEq)]
pub struct MerkleProof<N: AsRef<[u8]>> {
  pub hashes: Vec<Hash>,
  pub node_number: usize,
  pub index: usize,
  pub node: N,
}

impl<N: AsRef<[u8]>> Debug for MerkleProof<N> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("MerkleProof")
      .field("hashes", &hashes_to_str(&self.hashes))
      .field("node_number", &self.node_number)
      .field("index", &self.index)
      .field("node", &hash_to_str(&self.node.as_ref().to_vec()))
      .finish()
  }
}

pub trait MerkleTree {
  type Hasher: Hasher;

  fn new(hasher: Self::Hasher) -> Self;
  fn get_hasher(&self) -> &Self::Hasher;
  fn merkle_root<N: AsRef<[u8]>>(&self, leaves: &[N]) -> Hash;
  fn merkle_proof<N: AsRef<[u8]> + Clone>(&self, leaves: &[N], index: usize) -> Result<MerkleProof<N>, Error>;

  fn verify_proof<N: AsRef<[u8]>>(&self, root: &Hash, proof: &MerkleProof<N>) -> bool {
    let hasher = self.get_hasher();

    let mut result_hash = hasher.hash(&proof.node);
    let mut current_level_node_num = proof.node_number;
    let mut index = proof.index;
    let mut hashes = proof.hashes.clone();

    while current_level_node_num > 1 && !hashes.is_empty() {
      result_hash = if index % 2 != 0 {
        // `result_hash` is a right node.
        hasher.hash_two(hashes.remove(0), result_hash)
      } else if index != current_level_node_num - 1 {
        // `result_hash` is a left node AND not the last node
        hasher.hash_two(result_hash, hashes.remove(0))
      } else {
        result_hash
      };

      index /= 2;

      current_level_node_num =
        if current_level_node_num % 2 == 0 { current_level_node_num / 2 } else { current_level_node_num / 2 + 1 };
    }

    // A few housekeeping check
    if !hashes.is_empty() {
      false
    } else {
      root.to_vec() == result_hash
    }
  }
}
