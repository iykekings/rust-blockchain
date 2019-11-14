use sha2::Digest;

use crate::proofs::confirm_proof;

#[derive(Debug)]
pub struct BlockChain {
    pending_transactions: Vec<Transaction>, // will be Vec<Transaction>
    chain: Vec<Block> // will be Vec<Block>
}

impl BlockChain {
    pub fn create_genesis() -> BlockChain {
        let block = Block {
            index: 0,
            previous_hash: "123abc".to_string(),
            proof: 100,
            transactions: vec![]
        };

        BlockChain {
            pending_transactions: vec![],
            chain: vec![block]
        }
    }

}