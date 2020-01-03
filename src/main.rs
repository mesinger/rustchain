mod rustchain;

use rustchain::transaction::Transaction;
use crate::rustchain::hash::RustChainHash;

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

    println!("Transaction 1 Hash = {}", trans1.hash());
    println!("Transaction 2 Hash = {}", trans2.hash());
}
