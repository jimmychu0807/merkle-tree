extern crate merkle_tree;

use merkle_tree::{ MerkleTree };
// use blake2::Blake2s256;

fn main() {
  let data1 = vec![b"abc"];
  let data2 = vec![b"abc", b"bcd"];
  let data3 = vec![b"abc", b"bcd", b"cde"];
  let data4 = vec![b"abc", b"bcd", b"cde", b"def", b"efg"];

  let merkle_tree = MerkleTree::new();

  println!("{:?}", merkle_tree.merkle_root(data1.iter()));
}
