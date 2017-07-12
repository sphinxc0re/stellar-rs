use types::{PrivateKey, PublicKey};

#[derive(Clone)]
/// Holds a Stellar keypair.
pub struct KeyPair {
    public_key: PublicKey,
    private_key: Option<PrivateKey>,
}

impl KeyPair {
    /// Creates a new KeyPair without a private key. Useful to simply verify a signature from a
    /// given public address.
    pub fn from_pub(public_key: PublicKey) -> KeyPair {
        KeyPair {
            public_key,
            private_key: None,
        }
    }

    /// Creates a new KeyPair from the given public and private keys.
    pub fn from_pair(public_key: PublicKey, private_key: PrivateKey) -> KeyPair {
        KeyPair {
            public_key,
            private_key: Some(private_key),
        }
    }

    /// Returns true if this Keypair is capable of signing.
    pub fn can_sign(&self) -> bool {
        self.private_key.is_some()
    }
}
