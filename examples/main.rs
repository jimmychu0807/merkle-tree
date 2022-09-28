extern crate merkle_tree;

use merkle_tree::{BlakeTwo256Hasher, MerkleTreeIteration};

fn main() {
  let data1 = vec![b"abc"];
  let data2 = vec![b"abc", b"bcd"];
  let data7 = vec![b"a", b"b", b"c", b"d", b"e", b"f", b"g"];

  let merkle_tree = MerkleTreeIteration::new(BlakeTwo256Hasher::default());
  let root1 = merkle_tree.merkle_root(&data1);
  println!("merkle root: {:?}", hex::encode(&root1));

  let root2 = merkle_tree.merkle_root(&data2);
  println!("merkle root: {:?}", hex::encode(&root2));

  let root7 = merkle_tree.merkle_root(&data7);
  println!("merkle root: {:?}", hex::encode(&root7));

  let proof1_0 = merkle_tree.merkle_proof(&data1, 0);
  println!("merkle proof: {:?}", proof1_0);

  let proof2_1 = merkle_tree.merkle_proof(&data2, 0);
  println!("merkle proof: {:?}", proof2_1);

  let proof7_5 = merkle_tree.merkle_proof(&data7, 5);
  println!("merkle proof: {:?}", proof7_5);

  let proof7_6 = merkle_tree.merkle_proof(&data7, 6);
  println!("merkle proof: {:?}", proof7_6);
}
