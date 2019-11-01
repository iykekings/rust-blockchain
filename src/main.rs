fn main() {
    let b = BlockChain {
        pending_transactions: vec![1, 2],
        chain: vec![1, 2]
    };
    
    println!("{:#?}", b);
}

#[derive(Debug)]
struct BlockChain {
    pending_transactions: Vec<u32>, // will be Vec<Transaction>
    chain: Vec<u32> // will be Vec<Block>
}