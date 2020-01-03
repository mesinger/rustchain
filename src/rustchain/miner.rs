use crate::rustchain::chain::{BlockChain, Block, BlockHeader, blockHash};
use crate::rustchain::transaction::Transaction;
use crate::rustchain::merkletree::AsMerkleTree;
use crate::rustchain::hash::RustChainHash;

pub fn initializeBlock(blockChain: &BlockChain, transactions: Vec<Box<Transaction>>) -> Box<Block> {
    let header = BlockHeader{
        previousHash: blockChain.previousHash(),
        nonce: 0,
        target: blockChain.target(),
        transactionsHash: transactions.asMerkleTree().hash()
    };

    Box::new(Block {
        header,
        transactions
    })
}

//pub fn searchAndVerifyNonce<F : Fn(u128) -> u128>(block: &mut Box<Block>, nonceModifier : F) -> u128 {
//    while !block.verify() {
//        block.header.nonce = nonceModifier(block.header.nonce);
//        println!("nonce = {}", block.header.nonce);
//    }
//    block.header.nonce
//}

pub fn searchAndVerifyNonce<F : Fn(u128) -> u128>(previousHash : u128, initialNonce : u128, transactionHash : u128, target : u128, nonceModifier : F) -> u128 {
    let mut nonce = initialNonce;
    while blockHash(previousHash, nonce, transactionHash) > target {
        nonce = nonceModifier(nonce);
    }
    nonce
}
