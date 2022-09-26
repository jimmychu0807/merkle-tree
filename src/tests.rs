use crate::{
  MerkleTree,
  blake2_hasher::Blake2Hasher,
  sha3_hasher::Keccak256Hasher,
};

use blake2::{ Blake2s256, Digest };
use sha3::{ Keccak256 };

#[test]
fn one_level_root_using_blake2_hasher() {
  let data1 = vec![b"abc"];

  let tree = MerkleTree::new(Blake2Hasher::default());
  let root = tree.merkle_root(data1.iter());
  assert_eq!(root, Blake2s256::digest(b"abc").to_vec());
}

#[test]
fn one_level_root_using_keccak256_hasher() {
  let data1 = vec![b"abc"];

  let tree = MerkleTree::new(Keccak256Hasher::default());
  let root = tree.merkle_root(data1.iter());
  assert_eq!(root, Keccak256::digest(b"abc").to_vec());
}

