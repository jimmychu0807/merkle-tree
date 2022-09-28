use crate::Hash;

pub fn hash_to_str(input: &Hash) -> String {
  format!("0x{}", hex::encode(input))
}

pub fn hashes_to_str(inputs: &[Hash]) -> String {
  format!("[{}]", inputs.iter().map(hash_to_str).collect::<Vec<_>>().join(", "))
}
