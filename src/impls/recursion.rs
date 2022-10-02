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
    if leaves.len() == 1 {
      // Terminal case
      return self.hasher.hash(&leaves[0])
    }

    let left_node_num = find_left_node_num(leaves);

    let left_root = self.merkle_root(&leaves[..left_node_num]);
    #[cfg(feature = "logging")]
    log::trace!("left_root: {:?}", crate::utils::hash_to_str(&left_root));

    let right_root = self.merkle_root(&leaves[left_node_num..]);
    #[cfg(feature = "logging")]
    log::trace!("right_root: {:?}", crate::utils::hash_to_str(&right_root));

    self.hasher.hash_two(left_root, right_root)
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

fn find_left_node_num<N>(nodes: &[N]) -> usize {
  assert!(nodes.len() > 1, "number of nodes has to be larger than 1.");
  let mut power: usize = 1;

  while 2usize.pow(power as u32) < nodes.len() {
    power += 1;
  }

  // Here, 2^(power - 1) is the last highest 2 to the power number that is smaller than node.len()
  2usize.pow((power - 1) as u32)
}
