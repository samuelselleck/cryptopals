use crate::{algorithms, cryptobuff::CryptoBuff, stats};

#[test]
fn part_1() {
    let cb = CryptoBuff::new(b"YELLOW SUBMARINE");
    assert_eq!(
        cb.padded(20).to_vec(),
        b"YELLOW SUBMARINE\x04\x04\x04\x04".to_vec()
    );
}

#[test]
fn part_2() {
    let data = std::fs::read_to_string("data/set_2/part2.txt").unwrap();
    let conc = data.lines().collect::<String>();
    let cb = CryptoBuff::from_base64(&conc).unwrap();
    let cipher = Cipher::aes_128_ecb();
    let decrypted = decrypt(cipher, key, None, &cb).unwrap();
}
