// information provided by
// https://www.freecodecamp.org/news/how-bitcoin-mining-really-works-38563ec38c87/

use crate::rustchain::transaction::Transaction;
use crate::rustchain::hash::{RustChainHash, md5Hash};

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Box<Transaction>>
}

pub struct BlockHeader {
    pub previousHash: u128,
    pub nonce: u128,
    pub target: u128,
    pub transactionsHash: u128
}

pub struct BlockChain {
    blocks: Vec<Box<Block>>
}

impl Block {
    pub fn verify(&self) -> bool {
        self.hash() < self.header.target
    }
}

impl RustChainHash for Block {
    fn hash(&self) -> u128 {
        md5Hash(self.header.hash().to_le_bytes())
    }
}

impl RustChainHash for BlockHeader {
    fn hash(&self) -> u128 {
        md5Hash((self.previousHash + self.nonce as u128 + self.transactionsHash).to_le_bytes())
    }
}

pub fn blockHash(previousHash : u128, nonce : u128, transactionHash : u128) -> u128 {
    md5Hash(md5Hash((previousHash + nonce as u128 + transactionHash).to_le_bytes()).to_le_bytes())
}

impl BlockChain {

    pub fn new() -> BlockChain {
        BlockChain {
            blocks: vec![]
        }
    }

    pub fn addBlock(&mut self, block: Box<Block>) -> bool {
        if block.header.target != self.target() || block.verify() {
            self.blocks.push(block);
            true
        } else {
            false
        }
    }

    pub fn previousHash(&self) -> u128 {
        match self.blocks.last() {
            Some(block) => block.hash(),
            None => 0u128
        }
    }

    pub fn target(&self) -> u128 {
        0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFF
    }
}
