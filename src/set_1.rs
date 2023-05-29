use std::collections::HashMap;

use crate::{algorithms, cryptobuff::CryptoBuff, stats};
use openssl::symm::{decrypt, Cipher};

#[test]
fn part_1() {
    let test_str = "\
        49276d206b696c6c696e6720796f757220627261696e206c\
        696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let buff = CryptoBuff::from_hex(test_str).unwrap();
    let b64 = buff.to_base64();
    assert_eq!(
        b64,
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
}

#[test]
fn part_2() {
    let b1 = CryptoBuff::from_hex("1c0111001f010100061a024b53535009181c").unwrap();
    let b2 = CryptoBuff::from_hex("686974207468652062756c6c277320657965").unwrap();
    let res = (&b1 ^ &b2).to_hex();
    assert_eq!(res, "746865206b696420646f6e277420706c6179");
}

#[test]
fn part_3() {
    let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bs = CryptoBuff::from_hex(s).unwrap();
    let (guess, _) = algorithms::brute_force_single_byte_xor_ciper(&bs);
    let guess_str = guess.to_utf8();
    assert!(guess_str.is_ok());
    assert_eq!(guess_str.unwrap(), "Cooking MC's like a pound of bacon");
}

#[test]
fn part_4() {
    let data = std::fs::read_to_string("data/set_1/part4.txt").unwrap();
    let (_, best) = data
        .lines()
        .map(|l| {
            let cb = CryptoBuff::from_hex(l).unwrap();
            let (decrypted, _) = algorithms::brute_force_single_byte_xor_ciper(&cb);
            (stats::textlike_score(&decrypted), decrypted)
        })
        .min_by(|(v1, _), (v2, _)| v1.total_cmp(v2))
        .unwrap();
    let best_str = best.to_utf8();
    assert!(best_str.is_ok());
    assert_eq!(best_str.unwrap(), "Now that the party is jumping\n");
}

#[test]
fn part_5() {
    let text = b"Burning 'em, if you ain't quick and nimble\n\
                I go crazy when I hear a cymbal";
    let cb = CryptoBuff::new(text);
    let encoded = algorithms::repeating_key_xor(&cb, "ICE".as_bytes());
    assert_eq!(
        encoded.to_hex(),
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
             a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
    );
}

#[test]
fn part_6() {
    let data = std::fs::read_to_string("data/set_1/part6.txt").unwrap();
    let conc = data.lines().collect::<String>();
    let cb = CryptoBuff::from_base64(&conc).unwrap();
    let (_decrupted, key) = algorithms::break_repeating_xor_cypher(&cb);
    let key_str = key.to_utf8();
    assert!(key_str.is_ok());
    assert_eq!(key_str.unwrap(), "Terminator X: Bring the noise");
}

#[test]
fn part_7() {
    let data = std::fs::read_to_string("data/set_1/part7.txt").unwrap();
    let key = b"YELLOW SUBMARINE";
    let conc = data.lines().collect::<String>();
    let cb = CryptoBuff::from_base64(&conc).unwrap();
    let cipher = Cipher::aes_128_ecb();
    let decrypted = decrypt(cipher, key, None, &cb).unwrap();
    let expected_start = b"I'm back and I'm ringin' the bell";
    let txt_start = &decrypted[..expected_start.len()];
    assert_eq!(&txt_start, expected_start);
}

#[test]
fn part_8() {
    let data = std::fs::read_to_string("data/set_1/part8.txt").unwrap();
    let res = data
        .lines()
        .max_by_key(|l| {
            let cb = CryptoBuff::from_hex(l).unwrap();
            let mut freq = HashMap::new();
            for c in cb.array_chunks::<16>() {
                *freq.entry(c).or_default() += 1;
            }
            freq.values().map(|v| v - 1).sum::<i32>()
        })
        .unwrap();
    assert_eq!(
        res,
        "\
            d880619740a8a19b7840a8a31c810a3d\
            08649af70dc06f4fd5d2d69c744cd283\
            e2dd052f6b641dbf9d11b0348542bb57\
            08649af70dc06f4fd5d2d69c744cd283\
            9475c9dfdbc1d46597949d9c7e82bf5a\
            08649af70dc06f4fd5d2d69c744cd283\
            97a93eab8d6aecd566489154789a6b03\
            08649af70dc06f4fd5d2d69c744cd283\
            d403180c98c8f6db1f2a3f9c4040deb0\
            ab51b29933f2c123c58386b06fba186a"
    );
    println!("{}", res);
}
