extern crate merkle_tree;

use merkle_tree::{ MerkleTree, Blake2Hasher };

fn main() {
  let data1 = vec![b"abc"];
  let data2 = vec![b"abc", b"bcd"];
  let data3 = vec![b"abc", b"bcd", b"cde"];
  let data4 = vec![b"abc", b"bcd", b"cde", b"def", b"efg"];

  let merkle_tree = MerkleTree::new(Blake2Hasher::new());

  println!("{:?}", merkle_tree.merkle_root(data1.iter()));
}
