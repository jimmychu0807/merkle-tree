use blake2::{Blake2s256, Digest};

#[cfg(test)]
mod tests;

type Hash = Vec<u8>;

pub struct MerkleTree {}

impl MerkleTree {
	pub fn merkle_root<I: Iterator>(leaves: I) -> Hash {
		b"abcde".to_vec()
	}
}
