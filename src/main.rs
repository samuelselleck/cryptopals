#![feature(iter_array_chunks)]

mod algorithms;
mod cryptobuff;
mod stats;

use std::error::Error;

use crate::cryptobuff::CryptoBuff;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() {
    //part 1
    let test_str = "\
        49276d206b696c6c696e6720796f757220627261696e206c\
        696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let buff = CryptoBuff::from_hex(test_str).unwrap();
    println!("{}", buff.to_base64());

    //part 2
    let b1 = CryptoBuff::from_hex("1c0111001f010100061a024b53535009181c").unwrap();
    let b2 = CryptoBuff::from_hex("686974207468652062756c6c277320657965").unwrap();
    println!("{}", (&b1 ^ &b2).to_hex());

    //part 3
    let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bs = CryptoBuff::from_hex(s).unwrap();
    let (guess, _) = algorithms::brute_force_single_byte_xor_ciper(&bs);
    println!("{}", guess.to_utf8().unwrap());

    //part 4
    // let data = std::fs::read_to_string("data/set_1/part4.txt").unwrap();
    // let (best_guess, _) = data
    //     .lines()
    //     .map(|l| {
    //         let cb = CryptoBuff::from_hex(l).unwrap();
    //         let (decrypted, _) = algorithms::brute_force_single_byte_xor_ciper(&cb);
    //         (decrypted, stats::textlike_score(&cb))
    //     })
    //     .min_by(|(_, s1), (_, s2)| s1.total_cmp(s2))
    //     .unwrap();
    // println!("{}", best_guess.to_utf8().unwrap());

    //part 5
    let text = "Burning 'em, if you ain't quick and nimble \
                I go crazy when I hear a cymbal";
    let cb = CryptoBuff::new(text.as_bytes());
    let encoded = algorithms::repeating_key_xor(&cb, "ICE".as_bytes());
    println!("{}", encoded.to_hex());

    //part 6
    let data = std::fs::read_to_string("data/set_1/part6.txt").unwrap();
    let cb = CryptoBuff::from_base64(&data).unwrap();
}
