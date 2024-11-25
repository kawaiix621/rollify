use sha2::{Digest, Sha256};

/// Hash a given input using SHA-256
pub fn hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}

/// Generate a Merkle tree root from a list of transactions
pub fn generate_merkle_root(transactions: &[String]) -> String {
    transactions
        .iter()
        .map(|tx| hash(tx))
        .reduce(|acc, h| hash(&(acc + &h)))
        .unwrap_or_default()
}
