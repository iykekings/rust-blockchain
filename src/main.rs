use std::time::Instant;
mod blockchain;
mod proofs;

use crate::blockchain::{BlockChain, Transaction};
use crate::proofs::generate_proof;

fn main() {
    let now = Instant::now();
    let mut b = BlockChain::create_genesis();
    let new_transaction = Transaction::create("me".to_string(), "you".to_string(), 10);
    b.add_transaction(new_transaction);
    println!("{:#?}", b);
    let proof = generate_proof(&b);
    b.add_block(proof);
    println!("{:#?}", b);
    println!("Took: {:?}", now.elapsed());
}
