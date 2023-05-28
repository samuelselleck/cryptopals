#![feature(iter_array_chunks)]
pub mod algorithms;
pub mod cryptobuff;
pub mod stats;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[cfg(test)]
mod set_1 {

    use crate::{algorithms, cryptobuff::CryptoBuff, stats};

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
}
