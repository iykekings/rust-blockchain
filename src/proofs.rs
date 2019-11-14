use crate::blockchain::BlockChain;

pub fn confirm_proof(hash: &String) -> bool {
    &hash[..5] == "00000"
}

pub fn generate_proof(chain: &BlockChain) -> u32 {
    let mut proof = 0;
    let mut confirmed_hash = false;
    while confirmed_hash != true {
        proof += 1;
        let temp_hash = chain.hash(proof);
        if confirm_proof(&temp_hash) == true {
            confirmed_hash = true;
        }
    }
    proof
}