use types::{Integer, Long};
use key_pair::KeyPair;
use memo::Memo;
use operation::Operation;

const BASE_FEE: Integer = 100;

// Represents a <a href="https://www.stellar.org/developers/learn/concepts/transactions.html" target="_blank">Transaction</a> in the Stellar network
pub struct Transaction {
    fee: Integer,
    source_account: KeyPair,
    sequence_number: Long,
    operations: Vec<Operation>,
    memo: Memo,
    signatures: Vec<()>, // TODO: Port DecoratedSignatures
}

impl Transaction {
    pub fn new(source_account: KeyPair,
               sequence_number: Long,
               operations: Vec<Operation>,
               memo: Memo)
               -> Transaction {
        let num_operations = operations.len();

        Transaction {
            source_account,
            sequence_number,
            operations, // TODO: Check whether operations.len() > 0
            fee: num_operations as Integer * BASE_FEE,
            signatures: Vec::new(),
            memo,
        }
    }
}
