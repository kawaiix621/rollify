pub struct EthereumIntegration {
    pub node_url: String,
}

impl EthereumIntegration {
    pub fn new(node_url: &str) -> Self {
        EthereumIntegration {
            node_url: node_url.to_string(),
        }
    }

    pub fn submit_rollup(&self, data: &str) -> Result<String, String> {
        println!("Submitting rollup data to Ethereum node at {}", self.node_url);
        Ok(format!("Rollup hash: {}", data))
    }
}
