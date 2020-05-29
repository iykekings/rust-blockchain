use crate::blockchain::BlockChain;

pub fn confirm_proof(hash: &String) -> bool {
    &hash[..4] == "0000"
}

pub fn generate_proof(chain: &BlockChain) -> u32 {
    (0..)
    .skip_while(|&x| !confirm_proof(&chain.hash(x)))
    .next().unwrap() as u32
}
