extern crate merkle_tree;

use merkle_tree::{ MerkleTree };
// use blake2::Blake2s256;

fn main() {
  let data1 = vec![b"abc"];
  let data2 = vec![b"abc", b"bcd"];
  let data3 = vec![b"abc", b"bcd", b"cde"];
  let data4 = vec![b"abc", b"bcd", b"cde", b"def", b"efg"];

  let merkle_tree = MerkleTree::new();
  let root1 = merkle_tree.merkle_root(data1.iter());
  println!("merkle root: {:?}", hex::encode(&root1));

  let root2 = merkle_tree.merkle_root(data2.iter());
  println!("merkle root: {:?}", hex::encode(&root2));

  let root3 = merkle_tree.merkle_root(data3.iter());
  println!("merkle root: {:?}", hex::encode(&root3));

  let root4 = merkle_tree.merkle_root(data4.iter());
  println!("merkle root: {:?}", hex::encode(&root4));
}
