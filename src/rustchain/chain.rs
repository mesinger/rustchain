// information provided by
// https://www.freecodecamp.org/news/how-bitcoin-mining-really-works-38563ec38c87/

use crate::rustchain::transaction::Transaction;
use crate::rustchain::merkletree::MerkleTree;

struct Block {
    header: BlockHeader,
    transactions: Vec<Box<Transaction>>,
    hash: u128
}

struct BlockHeader {
    previousHash: u128,
    nonce: u16,
    target: u128,
    transactions: MerkleTree
}

pub struct BlockChain {
    blocks: Vec<Box<Block>>
}

impl BlockChain {

}
