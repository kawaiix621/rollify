pub struct ZKProof {
    pub statement: String,
    pub proof: String,
}

impl ZKProof {
    /// Generate a Zero-Knowledge Proof for a given statement
    pub fn generate(statement: &str) -> Self {
        let proof = format!("Proof of: {}", statement); // Simulated proof
        ZKProof {
            statement: statement.to_string(),
            proof,
        }
    }

    /// Verify a Zero-Knowledge Proof
    pub fn verify(&self) -> bool {
        self.proof.contains(&self.statement)
    }
}
