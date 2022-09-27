use core::fmt::{self, Debug};
use thiserror::Error;

#[cfg(test)]
mod tests;

// Re-export
mod traits;
pub use traits::{hash_to_str, hashes_to_str, Hash, Hasher};

// Re-export
pub mod blake2_hasher;
pub mod sha3_hasher;

pub struct MerkleTree<H: Hasher> {
  hasher: H,
}

#[derive(Clone)]
pub struct MerkleProof<N: AsRef<[u8]> + Clone> {
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

#[derive(Error, Debug)]
pub enum Error {
  #[error("Empty leaf")]
  EmptyLeaf,
  #[error("Index out of bound")]
  IndexOutOfBound,
  #[error("Unknown error")]
  Unknown,
}

impl<H: Hasher> MerkleTree<H> {
  pub fn new(hasher: H) -> Self {
    Self { hasher }
  }

  pub fn merkle_root<N>(&self, leaves: &[N]) -> Hash
  where
    N: AsRef<[u8]>,
  {
    // convert all data to hash
    let mut hashes: Vec<Hash> = leaves.iter().map(|l| self.hasher.hash(l)).collect();

    while hashes.len() > 1 {
      hashes = self.up_one_level(&hashes);
    }

    hashes[0].to_vec()
  }

  pub fn merkle_proof<N>(&self, leaves: &[N], index: usize) -> Result<MerkleProof<N>, Error>
  where
    N: AsRef<[u8]> + Clone,
  {
    // General checking
    if leaves.is_empty() {
      Err(Error::EmptyLeaf)
    } else if index >= leaves.len() {
      Err(Error::IndexOutOfBound)
    } else {
      Ok(())
    }?;

    let mut proof_hashes = vec![];
    let mut hashes: Vec<Hash> = leaves.iter().map(|l| self.hasher.hash(l).to_vec()).collect();

    let mut current_node = index;
    while hashes.len() > 1 {
      let current_level_node_num = hashes.len();

      if current_node % 2 == 0 && current_node != current_level_node_num - 1 {
        // Only push to `proof_hashes` when it is the left node but not the last
        //   node on that level.
        proof_hashes.push(hashes[current_node + 1].clone());
      } else if current_node % 2 != 0 {
        proof_hashes.push(hashes[current_node - 1].clone());
      }

      hashes = self.up_one_level(&hashes);
      current_node /= 2;
    }

    Ok(MerkleProof { hashes: proof_hashes, node_number: leaves.len(), index, node: leaves[index].clone() })
  }

  fn up_one_level(&self, input: &[Hash]) -> Vec<Hash> {
    let mut result: Vec<Hash> = vec![];
    let cnt = if input.len() % 2 == 0 { input.len() / 2 } else { input.len() / 2 + 1 };
    for i in 0..cnt {
      // Test that there is an odd number of node, and this is the last node.
      if input.len() % 2 != 0 && i * 2 == input.len() - 1 {
        result.push(input[i * 2].clone());
      } else {
        result.push(self.hasher.hash_two(&input[i * 2], &input[i * 2 + 1]));
      }
    }

    result
  }

  pub fn verify_proof<N>(&self, root: &Hash, proof: &MerkleProof<N>) -> bool
  where
    N: AsRef<[u8]> + Clone,
  {
    let mut result_hash = self.hasher.hash(&proof.node);
    let mut current_level_node_num = proof.node_number;
    let mut index = proof.index;
    let mut hashes = proof.hashes.clone();

    while current_level_node_num > 1 && !hashes.is_empty() {
      result_hash = if index % 2 != 0 {
        // `result_hash` is a right node.
        self.hasher.hash_two(hashes.remove(0), result_hash)
      } else if index != current_level_node_num - 1 {
        // `result_hash` is a left node AND not the last node
        self.hasher.hash_two(result_hash, hashes.remove(0))
      } else {
        result_hash
      };

      index /= 2;

      current_level_node_num =
        if current_level_node_num % 2 == 0 { current_level_node_num / 2 } else { current_level_node_num / 2 + 1 };
    }

    // A few housekeeping check
    // 1. There should be no more entry in `hashes`
    if !hashes.is_empty() {
      false
    } else {
      root.to_vec() == result_hash
    }
  }
}
