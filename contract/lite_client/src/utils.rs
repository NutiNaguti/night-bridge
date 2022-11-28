use std::{mem, num::ParseIntError};

use uint::unroll;

pub const BLOOM_BITS: u32 = 3;
pub const BLOOM_SIZE: usize = 256;

pub fn default() -> [u8; 256] {
    [0u8; 256]
}
pub fn accrue(bloom: &mut [u8; 256], input: [u8; 32]) {
    let p = BLOOM_BITS;

    let m = bloom.len();
    let bloom_bits = m * 8;
    let mask = bloom_bits - 1;
    let bloom_bytes = (log2(bloom_bits) + 7) / 8;

    let hash = near_sdk::env::keccak256(&input);

    // must be a power of 2
    assert_eq!(m & (m - 1), 0);
    // out of range
    assert!(p * bloom_bytes <= hash.len() as u32);

    let mut ptr = 0;

    assert_eq!(BLOOM_BITS, 3);
    unroll! {
        for i in 0..3 {
            let _ = i;
            let mut index = 0 as usize;
            for _ in 0..bloom_bytes {
                index = (index << 8) | hash[ptr] as usize;
                ptr += 1;
            }
            index &= mask;
            bloom[m - 1 - index / 8] |= 1 << (index % 8);
        }
    }
}

fn log2(x: usize) -> u32 {
    if x <= 1 {
        return 0;
    }

    let n = x.leading_zeros();
    mem::size_of::<usize>() as u32 * 8 - n
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
