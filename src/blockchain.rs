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
        let hash_str = self.hash(proof);
        let confirmed_proof = confirm_proof(&hash_str);
        
        if confirmed_proof {
           let transactions: Vec<_> =  self.pending_transactions.drain(..).collect();
            let block = Block::create(&self.chain.len(), hash_str, proof, transactions);
            &self.chain.push(block);
        } else {
            println!("Proof is wrong");
        }
        
    }

    pub fn hash(&self, proof: u32) -> String {
        let last_block_str = &self.last_block().unwrap().block_string();
    
        let result = sha2::Sha256::digest(format!("{}{}", last_block_str, proof).as_bytes());
        format!("{:x}", result)
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

#[derive(Debug)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: u32,
}

impl Transaction {
    pub fn create<T: Into<String>>(sender: T, receiver:  T, amount: u32) -> Transaction {
        Transaction {
            sender: sender.into(), 
            receiver: receiver.into(),
            amount
        }
    }
}
