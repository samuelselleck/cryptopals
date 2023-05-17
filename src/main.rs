#![feature(iter_array_chunks)]

use std::{error::Error, iter};

use itertools::Itertools;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() {
    let test_str = "\
        49276d206b696c6c696e6720796f757220627261696e206c\
        696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let res = hex_to_base64(test_str).unwrap();
    println!("{}", res);
}

fn to_base_64_char(n: u8) -> char {
    (match n {
        0..=25 => n + b'A',
        26..=51 => n - 26 + b'a',
        52..=61 => n - 52 + b'0',
        62 => b'+',
        63 => b'/',
        _ => panic!("not a base64 value"),
    } as char)
}

fn hex_to_base64(hex: &str) -> Result<String> {
    let infilled_chars = iter::repeat('0').take(hex.len() % 3).chain(hex.chars());

    let base16vals = infilled_chars
        .map(|c| c.to_digit(16).ok_or("not a hex digit"))
        .map_ok(|x| x as u8);

    let base64vals = base16vals
        .array_chunks()
        .map(|[a, b, c]| Ok([a?, b?, c?]))
        .map_ok(|[a, b, c]| [a << 2 | b >> 2, (b & 0b00000011) << 4 | c])
        .flatten_ok();

    base64vals.map_ok(to_base_64_char).collect()
}
