use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use serde_big_array::{self, BigArray};
use uint::unroll;

use crate::utils::log2;

pub const BLOOM_BITS: u32 = 3;
pub const BLOOM_SIZE: usize = 256;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Bloom {
    #[serde(with = "BigArray")]
    pub logs: [u8; 256],
}

impl From<[u8; 32]> for Bloom {
    fn from(input: [u8; 32]) -> Self {
        let mut bloom = Bloom::default();
        bloom.accrue(&input);
        bloom
    }
}

impl Bloom {
    pub fn default() -> Self {
        Self { logs: [0u8; 256] }
    }

    pub fn accrue(&mut self, input: &[u8; 32]) {
        let p = BLOOM_BITS;

        let m = self.logs.len();
        let bloom_bits = m * 8;
        let mask = bloom_bits - 1;
        let bloom_bytes = (log2(bloom_bits) + 7) / 8;

        // must be a power of 2
        assert_eq!(m & (m - 1), 0);
        // out of range
        assert!(p * bloom_bytes <= input.len() as u32);

        let mut ptr = 0;

        assert_eq!(BLOOM_BITS, 3);
        unroll! {
            for i in 0..3 {
                let _ = i;
                let mut index = 0 as usize;
                for _ in 0..bloom_bytes {
                    index = (index << 8) | input[ptr] as usize;
                    ptr += 1;
                }
                index &= mask;
                self.logs[m - 1 - index / 8] |= 1 << (index % 8);
            }
        }
    }

    pub fn contains_input(&self, input: [u8; 32]) -> bool {
        let bloom = input.into();
        self.contains_bloom(&bloom)
    }

    pub fn contains_bloom(&self, input: &Bloom) -> bool {
        for i in 0..BLOOM_SIZE {
            let a = self.logs[i];
            let b = input.logs[i];
            if (a & b) != b {
                return false;
            }
        }

        true
    }
}
