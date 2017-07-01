use types::Long;
use key_pair::KeyPair;
use transaction_builder_account::TransactionBuilderAccount;

// Represents an account in Stellar network with it's sequence number.
#[derive(Clone)]
pub struct Account {
    key_pair: KeyPair,
    sequence_number: Long,
}

impl Account {
    // Constructs a new `Account`
    pub fn new(key_pair: KeyPair, sequence_number: Long) -> Account {
        Account {
            key_pair,
            sequence_number,
        }
    }
}

impl TransactionBuilderAccount for Account {
    fn get_keypair(&self) -> KeyPair {
        self.key_pair.clone()
    }

    fn get_sequence_number(&self) -> Long {
        self.sequence_number
    }

    fn get_incremented_sequence_number(&self) -> Long {
        self.sequence_number + 1
    }

    fn increment_sequence_number(&mut self) {
        self.sequence_number += 1;
    }
}
