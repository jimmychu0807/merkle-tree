use blake2::{
  Blake2s256, Digest,
  digest::{ self, FixedOutputReset }
};

#[cfg(test)]
mod tests;

type Hash = Vec<u8>;

// pub trait Hasher {
//   fn hash<I: AsRef<u8>>(input: I) -> Hash;
//   fn hash_two<I: AsRef<u8>>(input1: I, input2: I) -> Hash;
// }

// pub struct Blake2Hasher {}

// impl Blake2Hasher {
//   pub fn new() -> Self {
//     Blake2Hasher {}
//   }
// }

// impl Hasher for Blake2Hasher {
//   fn hash<I: AsRef<u8>>(input: I) -> Hash {
//     Blake2s256::digest(input.as_ref()).to_vec()
//   }

//   fn hash_two<I: AsRef<u8>>(input1: I, input2: I) -> Hash {
//     Blake2s256::new()
//       .chain_update(input1)
//       .chain_update(input2)
//       .finalize()
//       .to_vec()
//   }
// }

pub struct MerkleTree {}

impl MerkleTree {
  pub fn new() -> Self {
    Self {}
  }

  pub fn merkle_root<I: Iterator>(&self, leaves: I) -> Hash
    where <I as Iterator>::Item: AsRef<[u8]>
  {
    // convert all data to hash
    let mut hashes: Vec<Hash> = leaves
      .map(|l| Blake2s256::digest(l).to_vec())
      .collect();

    while hashes.len() > 1 {
      let mut new_hashes: Vec<Hash> = vec![];
      let cnt = if hashes.len() % 2 == 0 {
        hashes.len() / 2
      } else {
        hashes.len() / 2 + 1
      };
      for i in 0..cnt {
        if i * 2 + 1 >= hashes.len() - 1 && hashes.len() % 2 != 0 {
          // This is the last node, and there is an odd number of node.
          new_hashes.push(hashes[i * 2].clone());
        } else {
          new_hashes.push(self.hash_two(&hashes[i * 2], &hashes[i * 2 + 1]));
        }
      }

      println!("hashes: {:?}", new_hashes);

      hashes = new_hashes;
    }

    hashes[0].to_vec()
  }

  fn hash_two(&self, input1: &[u8], input2: &[u8]) -> Hash {
    Blake2s256::new()
      .chain_update(input1)
      .chain_update(input2)
      .finalize()
      .to_vec()
  }
}
