use sha3::{Digest, Keccak256};
use secp256k1::{Secp256k1, Message, ecdsa::RecoverableSignature};
use primitive_types::H256;

pub fn keccak256(data: &[u8]) -> H256 {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    H256::from_slice(&hasher.finalize())
}

pub fn recover_signer(hash: &[u8; 32], v: u8, r: &[u8; 32], s: &[u8; 32]) -> Result<Vec<u8>, secp256k1::Error> {
    let secp = Secp256k1::new();
    let msg = Message::from_digest_slice(hash)?;
    
    // Construct recovery id (v is usually 27 or 28, or 0/1 for EIP-155 specific handling in core)
    // Assuming normalized v here (0 or 1)
    let recid = secp256k1::ecdsa::RecoveryId::from_i32(v as i32)?;
    
    let mut sig_bytes = [0u8; 64];
    sig_bytes[..32].copy_from_slice(r);
    sig_bytes[32..].copy_from_slice(s);
    
    let sig = RecoverableSignature::from_compact(&sig_bytes, recid)?;
    let pubkey = secp.recover_ecdsa(&msg, &sig)?;
    
    Ok(pubkey.serialize_uncompressed().to_vec())
}
