use std::collections::HashMap;

use crate::cryptobuff::CryptoBuff;

pub const CHAR_FREQUENCY: [(char, f64); 26] = [
    ('a', 0.082),
    ('b', 0.015),
    ('c', 0.028),
    ('d', 0.043),
    ('e', 0.13),
    ('f', 0.022),
    ('g', 0.02),
    ('h', 0.061),
    ('i', 0.07),
    ('j', 0.0015),
    ('k', 0.0077),
    ('l', 0.04),
    ('m', 0.024),
    ('n', 0.067),
    ('o', 0.075),
    ('p', 0.019),
    ('q', 0.00095),
    ('r', 0.06),
    ('s', 0.063),
    ('t', 0.091),
    ('u', 0.028),
    ('v', 0.0098),
    ('w', 0.024),
    ('x', 0.0015),
    ('y', 0.02),
    ('z', 0.00074),
];

pub fn textlike_score(cb: &CryptoBuff) -> f64 {
    let mut counts = HashMap::new();
    for c in cb.bytes() {
        *counts.entry(c.to_ascii_lowercase()).or_default() += 1;
    }

    let total_alph_chars: u32 = CHAR_FREQUENCY
        .into_iter()
        .map(|(c, _)| counts.get(&(c as u8)).unwrap_or(&0))
        .sum();

    let mut score = 0.0; //lower is better
    for (c, f) in CHAR_FREQUENCY {
        let count: u32 = counts.remove(&(c as u8)).unwrap_or_default();
        let dist = (count as f64) / (total_alph_chars as f64) - f;
        score += dist.abs();
    }
    score
}
