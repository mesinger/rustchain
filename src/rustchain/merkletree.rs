use crate::rustchain::hash::RustChainHash;

pub struct MerkleTree {
    pub root: Box<MerkleTreeNode>,
}

pub enum MerkleTreeNode {
    Empty,
    Leaf(u128),
    Tree(u128, Vec<Box<MerkleTreeNode>>)
}

pub trait AsMerkleTree {
    fn asMerkleTree(&self) -> MerkleTree;
}

impl RustChainHash for MerkleTree {
    fn hash(&self) -> u128 {
        self.root.hash()
    }
}

impl RustChainHash for MerkleTreeNode {
    fn hash(&self) -> u128 {
        match self {
            MerkleTreeNode::Leaf(hash) => *hash,
            MerkleTreeNode::Tree(hash, _) => *hash,
            MerkleTreeNode::Empty => 0 as u128
        }
    }
}
