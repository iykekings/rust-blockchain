use sha2::Digest;

use crate::proofs::confirm_proof;

#[derive(Debug)]
pub struct BlockChain {
    pending_transactions: Vec<Transaction>, // will be Vec<Transaction>
    chain: Vec<Block> // will be Vec<Block>
}

