use crate::rustchain::merkletree::{MerkleTreeNode, AsMerkleTree};
use crate::rustchain::hash::{RustChainHash, md5Hash};

pub struct Transaction {
    pub giver: u64,
    pub receiver: u64,
    pub amount: f64
}

impl AsMerkleTree for Vec<Box<Transaction>> {
    fn asMerkleTree(&self) -> MerkleTreeNode {
        match self.len() {
            0 => MerkleTreeNode::Empty,
            1 => MerkleTreeNode::Leaf(self.first().unwrap().hash()),
            _ => {
                let leafNodes : Vec<Box<MerkleTreeNode>> = self.iter().map(|transaction| Box::new(MerkleTreeNode::Leaf(transaction.hash()))).collect();
                let rootHash = md5Hash(leafNodes.iter().fold(0u128, |partHash, node| partHash ^ node.hash()).to_le_bytes());

                MerkleTreeNode::Root(rootHash, leafNodes)
            }
        }
    }
}

impl RustChainHash for Transaction {
    fn hash(&self) -> u128 {
        md5Hash((md5Hash(self.giver.to_le_bytes()) ^ md5Hash(self.receiver.to_le_bytes()) ^ md5Hash(self.amount.to_le_bytes())).to_le_bytes())
    }
}
