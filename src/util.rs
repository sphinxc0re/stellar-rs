use byte_sha;

pub fn hash(input: &mut Vec<u8>) -> Vec<u8> {
    let result: Vec<u8> = *byte_sha::sha256_of_message_as_u8_vec(input);

    result
}
