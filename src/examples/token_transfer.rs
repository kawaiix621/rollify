use rollify::core::{RollupManager, RollupType, Transaction};

fn main() {
    let manager = RollupManager::new(RollupType::ZKRollup, "https://ethereum-node");

    let tx = Transaction {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 50,
    };

    let batch = manager.process_batch(vec![tx]).unwrap();
    println!("Processed batch with {} transactions", batch.transactions.len());
}
