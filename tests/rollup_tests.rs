#[cfg(test)]
mod tests {
    use super::super::core::{RollupManager, RollupType, Transaction};

    #[test]
    fn test_process_batch() {
        let manager = RollupManager::new(RollupType::ZKRollup, "https://test-node");
        let tx = Transaction {
            from: "Alice".to_string(),
            to: "Bob".to_string(),
            amount: 100,
        };
        let result = manager.process_batch(vec![tx]);
        assert!(result.is_ok());
    }
}
