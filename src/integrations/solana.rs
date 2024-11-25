pub struct SolanaIntegration {
    pub rpc_url: String,
}

impl SolanaIntegration {
    pub fn new(rpc_url: &str) -> Self {
        SolanaIntegration {
            rpc_url: rpc_url.to_string(),
        }
    }

    pub fn submit_rollup(&self, data: &str) -> Result<String, String> {
        println!("Submitting rollup data to Solana RPC at {}", self.rpc_url);
        Ok(format!("Rollup submitted: {}", data))
    }
}
