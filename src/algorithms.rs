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
