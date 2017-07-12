use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn hash(input: &mut Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();

    hasher.input(&input[..]);

    let mut buf: [u8; 32] = [0; 32];

    hasher.result(&mut buf);

    buf.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        let mut input = String::from("hello world").into_bytes();

        assert_eq!(
            vec![
                185,
                77,
                39,
                185,
                147,
                77,
                62,
                8,
                165,
                46,
                82,
                215,
                218,
                125,
                171,
                250,
                196,
                132,
                239,
                227,
                122,
                83,
                128,
                238,
                144,
                136,
                247,
                172,
                226,
                239,
                205,
                233,
            ],
            hash(&mut input)
        )
    }
}
