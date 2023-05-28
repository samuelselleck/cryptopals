#![feature(iter_array_chunks)]

pub mod algorithms;
pub mod cryptobuff;
pub mod stats;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[cfg(test)]
mod set_1 {

    use crate::{algorithms, cryptobuff::CryptoBuff, stats};
    use base64::{engine::general_purpose, Engine as _};

    #[test]
    fn part_1() {
        let test_str = "\
        49276d206b696c6c696e6720796f757220627261696e206c\
        696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let buff = CryptoBuff::from_hex(test_str).unwrap();
        println!("{}", buff.to_base64());
    }

    #[test]
    fn part_2() {
        let b1 = CryptoBuff::from_hex("1c0111001f010100061a024b53535009181c").unwrap();
        let b2 = CryptoBuff::from_hex("686974207468652062756c6c277320657965").unwrap();
        println!("{}", (&b1 ^ &b2).to_hex());
    }

    #[test]
    fn part_3() {
        let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let bs = CryptoBuff::from_hex(s).unwrap();
        let (guess, _) = algorithms::brute_force_single_byte_xor_ciper(&bs);
        println!("part 3: {}", guess.to_utf8().unwrap());
    }

    #[test]
    fn part_4() {
        let data = std::fs::read_to_string("data/set_1/part4.txt").unwrap();
        let (_, best) = data
            .lines()
            .map(|l| {
                let bytes = hex::decode(l).unwrap();
                let cb = CryptoBuff::new(&bytes);
                let (decrypted, _) = algorithms::brute_force_single_byte_xor_ciper(&cb);
                (stats::textlike_score(&decrypted), decrypted)
            })
            .min_by(|(v1, _), (v2, _)| v1.total_cmp(v2))
            .unwrap();
        println!("{}", best.to_utf8().unwrap());
    }

    #[test]
    fn part_5() {
        let text = "Burning 'em, if you ain't quick and nimble \
                I go crazy when I hear a cymbal";
        let cb = CryptoBuff::new(text.as_bytes());
        let encoded = algorithms::repeating_key_xor(&cb, "ICE".as_bytes());
        println!("{}", encoded.to_hex());
    }

    #[test]
    fn part_6() {
        let data = std::fs::read_to_string("data/set_1/part6.txt").unwrap();
        let bytes = general_purpose::STANDARD
            .decode(data.lines().collect::<String>())
            .unwrap();
        let cb = CryptoBuff::new(&bytes);

        let (cb, key) = algorithms::break_repeating_xor_cypher(&cb);
        println!("{:?} {:?}", key.to_utf8(), cb.to_utf8());
    }
}

//TODO
// fix hamming_distance
// add asserts! to all tests
