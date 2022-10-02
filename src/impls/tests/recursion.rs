#![cfg(test)]

use crate::{MerkleTree, MerkleTreeIteration, MerkleTreeRecursion};

use blake2::{Blake2s256, Digest};

#[allow(dead_code)]
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
  let data = vec![b"a", b"b", b"c", b"d", b"e"];
  let tree_rec = MerkleTreeRecursion::default();
  let tree_itr = MerkleTreeIteration::default();
  assert_eq!(tree_rec.merkle_root(&data), tree_itr.merkle_root(&data));
}

#[test]
fn comparing_merkle_proof_of_seven_nodes_with_iteration() {
  init_test(|| {
    let data = vec![b"a", b"b", b"c", b"d", b"e", b"f", b"g"];
    let tree_rec = MerkleTreeRecursion::default();
    let tree_itr = MerkleTreeIteration::default();

    let proof_rec = tree_rec.merkle_proof(&data, 5);
    let proof_itr = tree_itr.merkle_proof(&data, 5);

    assert_eq!(proof_rec, proof_itr);
  })
}

#[test]
fn can_generate_root_proof_and_verify_for_non_lowest_tree_node() {
  let data = vec![b"a", b"b", b"c", b"d", b"e", b"f", b"g"];
  let tree = MerkleTreeRecursion::default();
  assert!(tree.verify_proof(&tree.merkle_root(&data), &tree.merkle_proof(&data, 6).unwrap()));
}
