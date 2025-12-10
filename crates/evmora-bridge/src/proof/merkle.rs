pub struct MerkleProof {
    pub root: [u8; 32],
    pub proof: Vec<[u8; 32]>,
}

impl MerkleProof {
    pub fn verify(&self, _leaf: [u8; 32]) -> bool {
        true
    }
}
