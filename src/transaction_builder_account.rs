use key_pair::KeyPair;
use types::Long;

pub trait TransactionBuilderAccount {
    // Returns the keypair associated with this Account
    fn get_keypair(&self) -> KeyPair;

    // Returns current sequence number of this Account.
    fn get_sequence_number(&self) -> Long;

    // Returns sequence number incremented by one, but does not increment internal counter.
    fn get_incremented_sequence_number(&self) -> Long;

    // Increments sequence number of this object by one.
    fn increment_sequence_number(&mut self);
}
