extern crate merkle_tree;

pub use merkle_tree::{MerkleTree};

fn main() {
  let data = [b"abc"];

  println!("Hello World!");
  println!("{:?}", MerkleTree::merkle_root(data.iter()));
}
