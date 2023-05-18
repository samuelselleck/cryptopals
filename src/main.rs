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

    let h1 = "1c0111001f010100061a024b53535009181c";
    let h2 = "686974207468652062756c6c277320657965";
    let res = hex_xor(h1, h2).unwrap();
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

fn hex_to_byte_vec(hex: &str) -> Result<Vec<u8>> {
    let mut vec = Vec::new();
    let s_offset = hex.len() % 2;
    if s_offset == 1 {
        vec.push(u8::from_str_radix(&hex[..1], 16)?);
    }

    for i in (s_offset..hex.len()).step_by(2) {
        vec.push(u8::from_str_radix(&hex[i..i + 2], 16)?);
    }
    Ok(vec)
}

fn byte_slice_to_hex(b: &[u8]) -> String {
    b.iter()
        .flat_map(|b| {
            [
                char::from_digit((b >> 4) as u32, 16).unwrap(),
                char::from_digit((b & 0b00001111) as u32, 16).unwrap(),
            ]
        })
        .collect()
}

fn byte_slice_xor(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    let n = b1.len();

    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        res.push(b1[i] ^ b2[i]);
    }
    res
}

fn hex_xor(h1: &str, h2: &str) -> Result<String> {
    let b1 = hex_to_byte_vec(h1)?;
    let b2 = hex_to_byte_vec(h2)?;
    let b_res = byte_slice_xor(&b1, &b2);
    Ok(byte_slice_to_hex(&b_res))
}
