use crate::aes::key_expansion::{add_round_key, key_expansion};
use crate::aes::mix_columns::{inverse_mix_columns, mix_columns};
use crate::aes::shift_rows::{inverse_shift_rows, shift_rows};
use crate::aes::substitution::{inverse_substitute, substitute};

mod key_expansion;
mod substitution;
mod mix_columns;
mod shift_rows;

pub(crate) const BLOCK_SIZE: usize = 16;
pub(crate) const NUM_ROUNDS: usize = 10;

pub fn cipher(input_block: [u8; BLOCK_SIZE], key: [u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
    let mut state = transform_byte_order(&input_block);
    let round_keys = key_expansion(&key);

    add_round_key(&mut state, &transform_byte_order(&round_keys[0]));

    for round in 1..NUM_ROUNDS {
        substitute(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, &transform_byte_order(&round_keys[round]));
    }

    substitute(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, &transform_byte_order(&round_keys[NUM_ROUNDS]));

    transform_byte_order(&state)
}

pub fn decipher(input_block: [u8; BLOCK_SIZE], key: [u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
    let mut state = transform_byte_order(&input_block);
    let round_keys = key_expansion(&key);

    add_round_key(&mut state, &transform_byte_order(&round_keys[NUM_ROUNDS]));
    inverse_shift_rows(&mut state);
    inverse_substitute(&mut state);

    for round in (1..NUM_ROUNDS).rev() {
        add_round_key(&mut state, &transform_byte_order(&round_keys[round]));
        inverse_mix_columns(&mut state);
        inverse_shift_rows(&mut state);
        inverse_substitute(&mut state);
    }

    add_round_key(&mut state, &transform_byte_order(&round_keys[0]));

    transform_byte_order(&state)
}

// transpose matrix to switch between row or column ordered block matrix
fn transform_byte_order(input: &[u8; 16]) -> [u8; 16] {
    let mut result = [0; 16];
    for i in 0..16 {
        let row = i % 4;
        let column = i / 4;
        result[row * 4 + column] = input[i];
    }

    result
}

#[test]
fn test_transpose() {
    let input: [u8; 16] = [0x1, 0x2, 0x3, 0x4,
                           0x5, 0x6, 0x7, 0x8,
                           0x9, 0xA, 0xB, 0xC,
                           0xD, 0xE, 0xF, 0x10];

    let result = transform_byte_order(&input);
    assert_eq!(result, [0x1, 0x5, 0x9, 0xD,
                        0x2, 0x6, 0xA, 0xE,
                        0x3, 0x7, 0xB, 0xF,
                        0x4, 0x8, 0xC, 0x10]);
}

#[test]
fn test_cipher() {
    let test_key: [u8; 16] = [0x54, 0x68, 0x61, 0x74, 0x73, 0x20, 0x6D, 0x79, 0x20, 0x4B, 0x75, 0x6E, 0x67, 0x20, 0x46, 0x75];
    let input: [u8; 16] = *b"Two One Nine Two";
    let cipher = cipher(input, test_key);

    // Test against Rust's AES library.
    use cipher::KeyInit;
    use cipher::BlockEncrypt;

    let mut block = cipher::generic_array::GenericArray::from(input);
    let cipher_aes = aes::Aes128::new(cipher::generic_array::GenericArray::from_slice(&test_key));
    cipher_aes.encrypt_block(&mut block);

    assert_eq!(cipher, [0x29, 0xC3, 0x50, 0x5F, 0x57, 0x14, 0x20, 0xF6, 0x40, 0x22, 0x99, 0xB3, 0x1A, 0x02, 0xD7, 0x3A]);
    assert_eq!(block.to_vec(), cipher);
}

#[test]
fn test_decipher() {
    let test_key: [u8; 16] = [0x54, 0x68, 0x61, 0x74, 0x73, 0x20, 0x6D, 0x79, 0x20, 0x4B, 0x75, 0x6E, 0x67, 0x20, 0x46, 0x75];
    let cipher: [u8; 16] = [0x29, 0xC3, 0x50, 0x5F, 0x57, 0x14, 0x20, 0xF6, 0x40, 0x22, 0x99, 0xB3, 0x1A, 0x02, 0xD7, 0x3A];
    let secret = decipher(cipher, test_key);

    // Test against Rust's AES library.
    use cipher::KeyInit;
    use cipher::BlockDecrypt;

    let mut block = cipher::generic_array::GenericArray::from(cipher);
    let cipher_aes = aes::Aes128::new(cipher::generic_array::GenericArray::from_slice(&test_key));
    cipher_aes.decrypt_block(&mut block);

    assert_eq!(block.to_vec(), *b"Two One Nine Two");
    assert_eq!(block.to_vec(), secret);
}