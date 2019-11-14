use crate::blockchain::BlockChain;

pub fn confirm_proof(hash: &String) -> bool {
    &hash[..5] == "00000"
}

