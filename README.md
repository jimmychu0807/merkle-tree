# Merkle Tree

Implement a [Merkle tree](https://en.wikipedia.org/wiki/Merkle_tree) and functions to verify proofs using it.

- Pick any popular [cryptographic hash function](https://en.wikipedia.org/wiki/Cryptographic_hash_function) library, such as BLAKE2 or Keccak. You don’t need to implement the hash function.
- Supports arbitrary number of leaves. They are initially hashed using the same hash function as the inner nodes. Inner nodes are created by concatenating child hashes and hashing again. The implementation does not need to perform any sorting of the input data (leaves).
- If the number of leaves is not even, the last leaf is promoted to the upper layer. So a 5-leaf Merkle tree look like this:

    ![tree.png](docs/assets/tree.png)

- Supports the following calls:
    - `fn new(hasher: Hasher) -> Self`
    - `fn merkle_root<N>(&self, leaves: &[N]) -> Hash`

        This function constructs a root hash of a Binary Merkle Tree created from given leaves

    - `fn merkle_proof(&self, leaves: &[N], leaf_index: usize) -> Result<MerkleProof, Error>`

        This function constructs a Merkle Proof for leaf specified by the index, by first constructing a (partial) Merkle Tree and return a `struct` storing all elements required to prove the leaf item, and contains the following:

        ```rust
        pub struct MerkleProof<N: AsRef<[u8]> + Clone> {
          pub hashes: Vec<Hash>,
          pub node_number: usize,
          pub index: usize,
          pub node: N
        }
        ```

    - `fn verify_proof(root: &Hash, proof: &MerkleProof) -> bool`

        This function verifies if the provided MerkleProof struct can finally generate back the Merkle `root` and prove itself.


## Usage Examples

```rust
let data = vec![b"abc", b"bcd", b"cde", b"def", b"efg"];
let tree = MerkleTree::new(BlakeTwo256Hasher::default());
let root = tree.merkle_root(&data);
let proof = tree.merkle_proof(&data, 1).unwrap();
assert!(tree.verify_proof(&root, &proof));
```

## References

- [https://en.wikipedia.org/wiki/Merkle_tree](https://en.wikipedia.org/wiki/Merkle_tree)
- [https://ethereum.org/en/developers/tutorials/merkle-proofs-for-offline-data-integrity/](https://ethereum.org/en/developers/tutorials/merkle-proofs-for-offline-data-integrity/)
