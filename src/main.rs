mod aes;

fn main() {
    let test_key: [u8; 16] = [0x54, 0x68, 0x61, 0x74, 0x73, 0x20, 0x6D, 0x79, 0x20, 0x4B, 0x75, 0x6E, 0x67, 0x20, 0x46, 0x75];
    let input: [u8; 16] = *b"Two One Nine Two";
    let cipher = aes::cipher(input, test_key);

    let decipher = aes::decipher(cipher, test_key);
    println!("{:?}", std::str::from_utf8(&input).unwrap());
    println!("{:#02x?}", cipher);
    println!("{:?}", std::str::from_utf8(&decipher).unwrap());

    todo!("Make it take key as HEX string as input.");
    todo!("Split longer input into 16 byte chunks.");
    todo!("AES 16 byte block padding logic.");
}
