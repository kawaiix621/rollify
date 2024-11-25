#[cfg(test)]
mod integration_tests {
    use super::super::integrations::ethereum::EthereumIntegration;

    #[test]
    fn test_ethereum_integration() {
        let ethereum = EthereumIntegration::new("https://test-ethereum-node");
        let result = ethereum.submit_rollup("test-data");
        assert!(result.is_ok());
    }
}
