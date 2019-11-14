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

    pub fn add_transaction(&mut self, transaction: Transaction) {
        &self.pending_transactions.push(transaction);
    }

    fn last_block(&self) -> Option<&Block> {
        match &self.chain.len() {
            0 => None,
            n => Some(&self.chain[n - 1])
        }
    }

    pub fn add_block(&mut self, proof: u32) {
        let transactions: Vec<Transaction> = self.pending_transactions
            .iter()
            .map(|Transaction { sender, receiver, amount }| {
                Transaction::create(
                    sender.clone(), 
                    receiver.clone(), 
                    amount.clone()
                )
            })
            .collect();

        let hash_str = self.hash(proof);

        let confirmed_proof = confirm_proof(&hash_str);
        
        if confirmed_proof == true {
            let block = Block::create(&self.chain.len(), hash_str, proof, transactions);
            &self.chain.push(block);
            self.pending_transactions.drain(..);
        } else {
            println!("Proof is wrong");
        }
        
    }

    pub fn hash(&self, proof: u32) -> String {
        let mut input = Vec::new();

        let last_block_str = &self.last_block().unwrap().block_string();
    
        for c in last_block_str.bytes() {
            input.push(c);
        }
    
        for c in proof.to_string().bytes() {
            input.push(c);
        }
    
        let result = sha2::Sha256::digest(&input);
        let result = format!("{:x}", result);
        result
    }
}

#[derive(Debug)]
struct Block {
    index: u32,
    previous_hash: String,
    proof: u32,
    transactions: Vec<Transaction>,
}

impl Block {
    fn create(index: &usize, previous_hash: String, proof: u32, transactions: Vec<Transaction>) -> Block {
        Block {
            index: *index as u32,
            previous_hash,
            proof, 
            transactions
        }
    }

    fn block_string(&self) -> String {
        let mut transactions = String::new();
        for t in  &self.transactions {
            transactions.push_str(&t.sender);
            transactions.push_str(&t.receiver);
            transactions.push_str(&t.amount.to_string()); 
        }

        transactions.push_str(&self.previous_hash);
        
        transactions
    }
}

