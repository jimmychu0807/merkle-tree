use crate::{blake2_hasher::BlakeTwo256Hasher, sha3_hasher::Keccak256Hasher, MerkleTree};

use blake2::{Blake2s256, Digest};
use sha3::Keccak256;

#[test]
fn one_level_root_using_blake2_hasher() {
  let data = vec![b"abc"];
  let tree = MerkleTree::new(BlakeTwo256Hasher::default());
  let root = tree.merkle_root(&data);
  assert_eq!(root, Blake2s256::digest(b"abc").to_vec());
}

#[test]
fn one_level_root_using_keccak256_hasher() {
  let data = vec![b"abc"];
  let tree = MerkleTree::new(Keccak256Hasher::default());
  let root = tree.merkle_root(&data);
  assert_eq!(root, Keccak256::digest(b"abc").to_vec());
}

#[test]
fn finding_merkle_root_of_five_nodes() {
  let data = vec![b"a", b"b", b"c", b"d", b"e"];
  let tree = MerkleTree::new(BlakeTwo256Hasher::default());
  let _root = tree.merkle_root(&data);
}

#[test]
fn can_generate_root_proof_and_verify() {
  let data = vec![b"a", b"b", b"c", b"d", b"e", b"f", b"g"];
  let tree = MerkleTree::default();
  assert!(tree.verify_proof(&tree.merkle_root(&data), &tree.merkle_proof(&data, 5).unwrap()));
}

#[test]
fn can_generate_root_proof_and_verify_for_non_lowest_tree_node() {
  let data = vec![b"a", b"b", b"c", b"d", b"e", b"f", b"g"];
  let tree = MerkleTree::default();
  assert!(tree.verify_proof(&tree.merkle_root(&data), &tree.merkle_proof(&data, 6).unwrap()));
}
