use util;

const PUBLIC: &str = "Public Global Stellar Network ; September 2015";
const TESTNET: &str = "Test SDF Network ; September 2015";

/// Network class is used to specify which Stellar network you want to use.
/// Each network has a `network_passphrase` which is hashed to every transaction id.
pub struct Network {
    network_passphrase: String,
}

impl Network {
    pub fn new(network_passphrase: String) -> Network {
        Network { network_passphrase }
    }

    // Returns network passphrase
    pub fn get_network_passphrase(&self) -> String {
        self.network_passphrase.clone()
    }

    // Returns network id (SHA-256 hashed `network_passphrase`).
    pub fn get_network_id(&self) -> Vec<u8> {
        let ref passphrase = self.network_passphrase;
        util::hash(&mut passphrase.as_bytes().into())
    }

    // Use Stellar Public Network
    pub fn public_network() -> Network {
        Network::new(String::from(PUBLIC))
    }

    // Use Stellar Test Network.
    pub fn test_network() -> Network {
        Network::new(String::from(TESTNET))
    }
}
