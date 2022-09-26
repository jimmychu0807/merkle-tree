pub type Hash = Vec<u8>;

pub trait Hasher {
  fn hash<I: AsRef<[u8]>>(&self, input: I) -> Hash;
  fn hash_two<I: AsRef<[u8]>>(&self, input1: I, input2: I) -> Hash;
}
