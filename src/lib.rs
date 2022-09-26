#[cfg(test)]
mod tests;

// Re-export
mod traits;
pub use traits::{ Hash, Hasher };

// Re-export
pub mod blake2_hasher;
pub mod sha3_hasher;

pub struct MerkleTree<H: Hasher> {
  hasher: H,
}

impl<H: Hasher> MerkleTree<H> {
  pub fn new(hasher: H) -> Self {
    Self { hasher }
  }

  pub fn merkle_root<I: Iterator>(&self, leaves: I) -> Hash
  where
    <I as Iterator>::Item: AsRef<[u8]>,
  {
    // convert all data to hash
    let mut hashes: Vec<Hash> = leaves
      .map(|l| self.hasher.hash(l))
      .collect();

    while hashes.len() > 1 {
      let mut new_hashes: Vec<Hash> = vec![];
      let cnt = if hashes.len() % 2 == 0 { hashes.len() / 2 } else { hashes.len() / 2 + 1 };
      for i in 0..cnt {
        if i * 2 + 1 >= hashes.len() - 1 && hashes.len() % 2 != 0 {
          // This is the last node, and there is an odd number of node.
          new_hashes.push(hashes[i * 2].clone());
        } else {
          new_hashes.push(self.hasher.hash_two(&hashes[i * 2], &hashes[i * 2 + 1]));
        }
      }

      println!("hashes: {:?}", new_hashes);

      hashes = new_hashes;
    }

    hashes[0].to_vec()
  }
}
