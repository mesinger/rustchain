use crate::rustchain::hash::RustChainHash;

pub enum MerkleTreeNode {
    Empty,
    Leaf(u128),
    Root(u128, Vec<Box<MerkleTreeNode>>)
}

pub trait AsMerkleTree {
    fn asMerkleTree(&self) -> MerkleTreeNode;
}

impl RustChainHash for MerkleTreeNode {
    fn hash(&self) -> u128 {
        match self {
            MerkleTreeNode::Leaf(hash) => *hash,
            MerkleTreeNode::Root(hash, _) => *hash,
            MerkleTreeNode::Empty => 0 as u128
        }
    }
}
