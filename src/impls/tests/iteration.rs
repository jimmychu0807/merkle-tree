#![cfg(test)]

use crate::{
  hashers::{BlakeTwo256Hasher, Keccak256Hasher},
  MerkleTree, MerkleTreeIteration,
};

use blake2::{Blake2s256, Digest};
use sha3_lib::Keccak256;

fn init_test(f: fn()) {
  #[cfg(feature = "logging")]
  env_logger::try_init();

  f();
}

#[test]
fn one_level_root_using_blake2_hasher() {
  let data = vec![b"abc"];
  let tree = MerkleTreeIteration::new(BlakeTwo256Hasher::default());
  let root = tree.merkle_root(&data);
  assert_eq!(root, Blake2s256::digest(b"abc").to_vec());
}

#[test]
fn one_level_root_using_keccak256_hasher() {
  let data = vec![b"abc"];
  let tree = MerkleTreeIteration::new(Keccak256Hasher::default());
  let root = tree.merkle_root(&data);
  assert_eq!(root, Keccak256::digest(b"abc").to_vec());
}

#[test]
fn finding_merkle_root_of_five_nodes() {
  let data = vec![b"a", b"b", b"c", b"d", b"e"];
  let tree = MerkleTreeIteration::new(BlakeTwo256Hasher::default());
  let _root = tree.merkle_root(&data);
}

#[test]
fn can_generate_root_proof_and_verify() {
  let data = vec![b"a", b"b", b"c", b"d", b"e", b"f", b"g"];
  let tree = MerkleTreeIteration::default();
  assert!(tree.verify_proof(&tree.merkle_root(&data), &tree.merkle_proof(&data, 5).unwrap()));
}

#[test]
fn can_generate_root_proof_and_verify_for_non_lowest_tree_node() {
  let data = vec![b"a", b"b", b"c", b"d", b"e", b"f", b"g"];
  let tree = MerkleTreeIteration::default();
  assert!(tree.verify_proof(&tree.merkle_root(&data), &tree.merkle_proof(&data, 6).unwrap()));
}
