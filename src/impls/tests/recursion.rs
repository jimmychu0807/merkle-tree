#![cfg(test)]

use crate::{
  hashers::BlakeTwo256Hasher,
  MerkleTree, MerkleTreeIteration, MerkleTreeRecursion,
};

use blake2::{Blake2s256, Digest};

fn init_test(f: fn()) {
  #[cfg(feature = "logging")]
  env_logger::try_init();
  f();
}

#[test]
fn finding_merkle_root_of_one_node() {
  let data = vec![b"abc"];
  let tree = MerkleTreeRecursion::default();
  let root = tree.merkle_root(&data);
  assert_eq!(root, Blake2s256::digest(b"abc").to_vec());
}

#[test]
fn comparing_merkle_root_of_five_nodes_with_iteration() {
  init_test(|| {
    let data = vec![b"a", b"b", b"c", b"d", b"e"];
    let tree_rec = MerkleTreeRecursion::default();
    let tree_itr = MerkleTreeIteration::default();
    assert_eq!(tree_rec.merkle_root(&data), tree_itr.merkle_root(&data));
  })
}
