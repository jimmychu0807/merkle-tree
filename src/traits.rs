pub type Hash = Vec<u8>;

pub fn hash_to_str(input: &Hash) -> String {
  format!("0x{}", hex::encode(input))
}

pub fn hashes_to_str(inputs: &[Hash]) -> String {
  format!("[{}]", inputs.iter().map(hash_to_str).collect::<Vec<_>>().join(", "))
}

pub trait Hasher {
  fn hash<I: AsRef<[u8]>>(&self, input: I) -> Hash;
  fn hash_two<I: AsRef<[u8]>>(&self, input1: I, input2: I) -> Hash;
}
