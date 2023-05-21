use std::ops::Deref;

use super::Result;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct CryptoBuff {
    bytes: Vec<u8>,
}

impl Deref for CryptoBuff {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl From<Vec<u8>> for CryptoBuff {
    fn from(value: Vec<u8>) -> Self {
        Self { bytes: value }
    }
}

impl FromIterator<u8> for CryptoBuff {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let bytes: Vec<u8> = iter.into_iter().collect();
        Self { bytes }
    }
}

impl std::ops::BitXor for &CryptoBuff {
    type Output = CryptoBuff;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let n = std::cmp::min(self.bytes().len(), rhs.bytes().len());

        let mut bytes = Vec::with_capacity(n);
        for i in 0..n {
            bytes.push(self.bytes()[i] ^ rhs.bytes()[i]);
        }
        Self::Output { bytes }
    }
}

impl CryptoBuff {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
        }
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        let mut data = Vec::new();
        let s_offset = hex.len() % 2;
        if s_offset == 1 {
            data.push(u8::from_str_radix(&hex[..1], 16)?);
        }

        for i in (s_offset..hex.len()).step_by(2) {
            data.push(u8::from_str_radix(&hex[i..i + 2], 16)?);
        }
        Ok(Self { bytes: data })
    }

    pub fn from_base64(base64: &str) -> Result<Self> {
        fn from_base64_char(c: char) -> Result<u8> {
            if !c.is_ascii() {
                return Err("detected non-ascii character".into());
            }

            let v = c as u8;
            Ok(match c {
                'a'..='z' => v - b'a' + 26,
                'A'..='Z' => v - b'A',
                '0'..='9' => v - b'0' + 52,
                '+' => 62,
                '/' => 63,
                _ => return Err("not a base64 character".into()),
            })
        }

        let vals: Result<Vec<u8>> = base64.chars().map(from_base64_char).collect();

        #[rustfmt::skip]
        let bytes = vals?
            .into_iter()
            .array_chunks()
            .flat_map(|[a, b, c, d]| {
                [
                    (a << 2) | (b >> 4), 
                    (b << 4) | (c >> 2), 
                    (c << 6) | d
                ]
            })
            .collect();

        Ok(Self { bytes })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_base64(&self) -> String {
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

        self.bytes()
            .iter()
            .array_chunks()
            .flat_map(|[a, b, c]| {
                [
                    a >> 2,
                    ((a & 0b11) << 4) | (b >> 4),
                    ((b & 0b1111) << 2) | (c >> 6),
                    c & 0b111111,
                ]
            })
            .map(to_base_64_char)
            .collect()
    }

    pub fn to_hex(&self) -> String {
        self.bytes()
            .iter()
            .flat_map(|b| {
                [
                    char::from_digit((b >> 4) as u32, 16).unwrap(),
                    char::from_digit((b & 0b00001111) as u32, 16).unwrap(),
                ]
            })
            .collect()
    }

    pub fn to_utf8(&self) -> Result<String> {
        Ok(String::from_utf8(self.bytes().to_vec())?)
    }
}
