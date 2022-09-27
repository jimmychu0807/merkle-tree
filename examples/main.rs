extern crate merkle_tree;

use merkle_tree::{
  MerkleTree,
  blake2_hasher::Blake2Hasher,
};

fn main() {
  let data1 = vec![b"abc"];
  let data2 = vec![b"abc", b"bcd"];
  let data3 = vec![b"abc", b"bcd", b"cde"];
  let data7 = vec![b"a", b"b", b"c", b"d", b"e", b"f", b"g"];

  let merkle_tree = MerkleTree::new(Blake2Hasher::default());
  let root1 = merkle_tree.merkle_root(&data1);
  println!("merkle root: {:?}", hex::encode(&root1));

  let root2 = merkle_tree.merkle_root(&data2);
  println!("merkle root: {:?}", hex::encode(&root2));

  let root7 = merkle_tree.merkle_root(&data7);
  println!("merkle root: {:?}", hex::encode(&root7));

  let proof7_5 = merkle_tree.merkle_proof(&data7, 5);
  println!("merkle proof: {:?}", proof7_5);
}
