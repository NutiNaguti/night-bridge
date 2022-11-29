use std::{mem, num::ParseIntError};

pub fn log2(x: usize) -> u32 {
    if x <= 1 {
        return 0;
    }

    let n = x.leading_zeros();
    mem::size_of::<usize>() as u32 * 8 - n
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
