pub struct OptimisticState {
    pub state_root: String,
}

impl OptimisticState {
    /// Create a new OptimisticState
    pub fn new(state_root: &str) -> Self {
        OptimisticState {
            state_root: state_root.to_string(),
        }
    }

    /// Simulate fraud-proof validation
    pub fn validate_fraud_proof(&self, fraud_proof: &str) -> bool {
        fraud_proof.contains(&self.state_root)
    }
}
