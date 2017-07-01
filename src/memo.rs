/// The memo contains optional extra information. It is the responsibility of the client to interpret this value
pub enum Memo {
    /// Empty memo
    None,
    /// A string up to 28-bytes long
    Text([u8; 28]),
    /// A 64 bit unsigned integer
    ID(u64),
    /// A 32 byte hash
    Hash([u8; 32]),
    /// A 32 byte hash intended to be interpreted as the hash of the transaction the sender is refunding
    Return([u8; 32]),
}
