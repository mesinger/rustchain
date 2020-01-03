mod rustchain;

use rustchain::transaction::Transaction;
use crate::rustchain::miner::{initializeBlock, searchAndVerifyNonce};
use crate::rustchain::chain::BlockChain;

fn main() {
    let trans1 = Box::new(Transaction{
        giver: 1,
        receiver: 2,
        amount: 1.0
    });

    let trans2 = Box::new(Transaction{
        giver: 2,
        receiver: 1,
        amount: 0.5
    });

    let transactions = vec![trans1, trans2];

    let mut blockChain = BlockChain::new();
    let mut block = initializeBlock(&blockChain, transactions);

    let verifiedNonce = searchAndVerifyNonce(block.header.previousHash, block.header.nonce, block.header.transactionsHash, block.header.target, |nonce| nonce + 1);
    println!("nonce verified = {}", verifiedNonce);
    block.header.nonce = verifiedNonce;
    println!("blockchain accepted block = {}", blockChain.addBlock(block));
}
