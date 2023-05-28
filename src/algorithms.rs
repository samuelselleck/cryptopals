use crate::{cryptobuff::CryptoBuff, stats};

pub fn brute_force_single_byte_xor_ciper(cb: &CryptoBuff) -> (CryptoBuff, u8) {
    (0..=u8::MAX)
        .map(|code| {
            let decoded = cb.iter().map(|v| v ^ code).collect();
            (decoded, code)
        })
        .min_by(|(d1, _), (d2, _)| {
            let s1 = stats::textlike_score(d1);
            let s2 = stats::textlike_score(d2);
            s1.total_cmp(&s2)
        })
        .unwrap()
}

pub fn repeating_key_xor(cb: &CryptoBuff, key: &[u8]) -> CryptoBuff {
    key.iter()
        .cycle()
        .zip(cb.iter())
        .map(|(v, u)| v ^ u)
        .collect()
}

pub fn break_repeating_xor_cypher(cb: &CryptoBuff) -> (CryptoBuff, CryptoBuff) {
    let lens = guess_keylen(cb);
    let (key, decoded) = lens
        .iter()
        .map(|&len| {
            let key = guess_decode_key(cb, len);
            let decoded = repeating_key_xor(cb, &key);
            (key, decoded)
        })
        .min_by(|(_, d1), (_, d2)| {
            let score1 = stats::textlike_score(d1);
            let score2 = stats::textlike_score(d2);
            score1.total_cmp(&score2)
        })
        .unwrap();
    (decoded, key)
}

fn hamming_distance(d1: &[u8], d2: &[u8]) -> u32 {
    d1.iter().zip(d2).map(|(a, b)| (a ^ b).count_ones()).sum()
}

fn guess_keylen(data: &CryptoBuff) -> Vec<u32> {
    let mut key_sizes: Vec<u32> = (2..=40).collect();

    key_sizes.sort_by_key(|&keylen| {
        let k_l = keylen as usize;
        hamming_distance(&data[0..k_l], &data[k_l..(2 * k_l)]) / keylen
    });

    key_sizes.to_vec()
}

fn guess_decode_key(cb: &CryptoBuff, key_size: u32) -> CryptoBuff {
    let blocks: Vec<_> = cb.chunks(key_size as usize).collect();

    (0..key_size)
        .map(|i| {
            let column: CryptoBuff = blocks
                .iter()
                .filter_map(|word| word.get(i as usize))
                .copied()
                .collect();
            let (_, res) = brute_force_single_byte_xor_ciper(&column);
            res
        })
        .collect()
}
