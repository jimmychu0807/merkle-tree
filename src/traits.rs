pub type Hash = Vec<u8>;

pub fn hash_to_str(input: &Hash) -> String {
  format!("0x{}", hex::encode(input))
}

pub fn hashes_to_str(inputs: &[Hash]) -> String {
  format!("[{}]", inputs.iter().map(hash_to_str).collect::<Vec<_>>().join(", "))
}

pub trait Hasher {
  fn hash<I: AsRef<[u8]>>(&self, input: I) -> Hash;

  fn hash_two<I: AsRef<[u8]>>(&self, input1: I, input2: I) -> Hash {
    let input1_ref = input1.as_ref();
    let input2_ref = input2.as_ref();

    let mut buf = vec![0; input1_ref.len() + input2_ref.len()];
    buf[..input1_ref.len()].clone_from_slice(input1_ref);
    buf[input1_ref.len()..].clone_from_slice(input2_ref);
    self.hash(&buf)
  }
}
