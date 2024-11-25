use rollify::core::{RollupManager, RollupType};

fn main() {
    let manager = RollupManager::new(RollupType::OptimisticRollup, "https://solana-node");

    println!("Minting NFTs using Optimistic Rollup...");
}
