pub(crate) fn mix_columns(input: &mut [u8; 16]) {
    for i in 0..4 {
        let s0 = input[i];
        let s1 = input[i + 4];
        let s2 = input[i + 8];
        let s3 = input[i + 12];

        input[i] = mul2(s0) ^ mul3(s1) ^ s2 ^ s3;
        input[i + 4] = s0 ^ mul2(s1) ^ mul3(s2) ^ s3;
        input[i + 8] = s0 ^ s1 ^ mul2(s2) ^ mul3(s3);
        input[i + 12] = mul3(s0) ^ s1 ^ s2 ^ mul2(s3);
    }
}

pub(crate) fn inverse_mix_columns(input: &mut [u8; 16]) {
    for i in 0..4 {
        let s0 = input[i];
        let s1 = input[i + 4];
        let s2 = input[i + 8];
        let s3 = input[i + 12];

        input[i] = mul14(s0) ^ mul11(s1) ^ mul13(s2) ^ mul9(s3);
        input[i + 4] = mul9(s0) ^ mul14(s1) ^ mul11(s2) ^ mul13(s3);
        input[i + 8] = mul13(s0) ^ mul9(s1) ^ mul14(s2) ^ mul11(s3);
        input[i + 12] = mul11(s0) ^ mul13(s1) ^ mul9(s2) ^ mul14(s3);

    }
}

fn mul2(byte: u8) -> u8 {
    if byte & 0x80 != 0 {
        (byte << 1) ^ 0x1b
    } else {
        byte << 1
    }
}

fn mul3(byte: u8) -> u8 {
    mul2(byte) ^ byte
}

fn mul9(byte: u8) -> u8 {
    mul2(mul2(mul2(byte))) ^ byte
}

fn mul11(byte: u8) -> u8 {
    mul2(mul2(mul2(byte)) ^ byte) ^ byte
}

fn mul13(byte: u8) -> u8 {
    mul2(mul2(mul2(byte) ^ byte)) ^ byte
}

fn mul14(byte: u8) -> u8 {
    mul2(mul2(mul2(byte) ^ byte) ^ byte)
}

#[test]
fn test_mix_columns() {
    let mut input: [u8; 16] = [0x33, 0x51, 0x79, 0x0A, 0x8B, 0x66, 0x8F, 0x3F, 0x76, 0x7D, 0xEB, 0xBE, 0x20, 0x92, 0xC2, 0x67];
    mix_columns(&mut input);
    assert_eq!(input, [0xB6, 0xE7, 0x51, 0x8C, 0x84, 0x88, 0x98, 0xCA, 0x34, 0x60, 0x66, 0xFB, 0xE8, 0xD7, 0x70, 0x51]);
}

#[test]
fn test_inverse_mix_columns() {
    let mut input: [u8; 16] = [0xB6, 0xE7, 0x51, 0x8C, 0x84, 0x88, 0x98, 0xCA, 0x34, 0x60, 0x66, 0xFB, 0xE8, 0xD7, 0x70, 0x51];
    inverse_mix_columns(&mut input);
    assert_eq!(input, [0x33, 0x51, 0x79, 0x0A, 0x8B, 0x66, 0x8F, 0x3F, 0x76, 0x7D, 0xEB, 0xBE, 0x20, 0x92, 0xC2, 0x67]);
}