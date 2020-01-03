use crate::rustchain::merkletree::{MerkleTree, MerkleTreeNode, AsMerkleTree};
use crate::rustchain::hash::{RustChainHash, md5Hash};

pub struct Transaction {
    pub giver: u64,
    pub receiver: u64,
    pub amount: f64
}

impl AsMerkleTree for Vec<Transaction> {
    fn asMerkleTree(&self) -> MerkleTree {
        let leafNodes : Vec<Box<MerkleTreeNode>> = self.iter().map(|transaction| Box::new(MerkleTreeNode::Leaf(transaction.hash()))).collect();
        let rootHash = md5Hash(leafNodes.iter().fold(0u128, |partHash, node| partHash ^ node.hash()).to_le_bytes());

        MerkleTree {
            root: Box::new(MerkleTreeNode::Tree(rootHash, leafNodes))
        }
    }
}

impl RustChainHash for Transaction {
    fn hash(&self) -> u128 {
        md5Hash(self.giver.to_le_bytes()) ^ md5Hash(self.receiver.to_le_bytes()) ^ md5Hash(self.amount.to_le_bytes())
    }
}
