extern crate merkle_tree;

use blake2::Blake2s256;
use merkle_tree::MerkleTree;

fn main() {
  let data = vec![b"abc", b"bcd", b"cde", b"def", b"efg"];

  let merkle_tree = MerkleTree::new(Blake2s256::new());

  println!("{:?}", MerkleTree::merkle_root(data.iter()));
}
