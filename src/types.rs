pub type Long = u64;
pub type Integer = i32;

#[derive(Clone)]
pub struct PublicKey([u8; 32]);

pub struct PrivateKey([u8; 64]);

impl Clone for PrivateKey {
    fn clone(&self) -> Self {
        PrivateKey(self.0)
    }
}
