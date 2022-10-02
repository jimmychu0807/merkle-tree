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

  fn get_hasher(&self) -> &Self::Hasher {
    &self.hasher
  }

  fn merkle_root<N: AsRef<[u8]>>(&self, leaves: &[N]) -> Hash {
    if leaves.len() == 1 {
      // Terminal case
      return self.hasher.hash(&leaves[0])
    }

    let left_node_num = find_left_node_num(leaves);
    let left_root = self.merkle_root(&leaves[..left_node_num]);
    let right_root = self.merkle_root(&leaves[left_node_num..]);
    self.hasher.hash_two(left_root, right_root)
  }

  fn merkle_proof<N: AsRef<[u8]> + Clone>(&self, leaves: &[N], index: usize) -> Result<MerkleProof<N>, Error> {
    let mut hashes = vec![];

    self.merkle_proof_rec(leaves, Some(index), &mut hashes);

    Ok(MerkleProof { hashes, node_number: leaves.len(), index, node: leaves[index].clone() })
  }
}

impl<H: Hasher> MerkleTreeRecursion<H> {
  fn merkle_proof_rec<N: AsRef<[u8]> + Clone>(
    &self,
    leaves: &[N],
    target: Option<usize>,
    hashes: &mut Vec<Hash>,
  ) -> Hash {
    if leaves.len() == 1 {
      return self.hasher.hash(&leaves[0])
    }

    let left_node_num = find_left_node_num(leaves);

    let left_tree_target = if let Some(target) = target {
      if target < left_node_num {
        Some(target)
      } else {
        None
      }
    } else {
      None
    };

    let left_root = self.merkle_proof_rec(&leaves[..left_node_num], left_tree_target, hashes);

    let right_tree_target = if let Some(target) = target {
      if target >= left_node_num {
        Some(target - left_node_num)
      } else {
        None
      }
    } else {
      None
    };

    let right_root = self.merkle_proof_rec(&leaves[left_node_num..], right_tree_target, hashes);

    match target {
      Some(target) => hashes.push(if target < left_node_num { right_root.clone() } else { left_root.clone() }),
      None => {}
    }

    self.hasher.hash_two(left_root, right_root)
  }
}

impl Default for MerkleTreeRecursion<BlakeTwo256Hasher> {
  fn default() -> Self {
    Self::new(BlakeTwo256Hasher::default())
  }
}

fn find_left_node_num<N>(nodes: &[N]) -> usize {
  assert!(nodes.len() > 1, "number of nodes has to be larger than 1.");
  let mut power: usize = 1;

  while 2usize.pow(power as u32) < nodes.len() {
    power += 1;
  }

  // Here, 2^(power - 1) is the last highest 2 to the power number that is smaller than node.len()
  2usize.pow((power - 1) as u32)
}
