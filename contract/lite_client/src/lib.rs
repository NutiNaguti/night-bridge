use near_sdk::{
    self, base64,
    borsh::{self, BorshDeserialize, BorshSerialize},
    store::UnorderedMap,
    PanicOnDefault,
};
use near_sdk::{near_bindgen, BorshStorageKey};
use serde::{Deserialize, Serialize};
use serde_big_array::{self, BigArray};

mod utils;

type BlockNumber = u64;

#[derive(Serialize, Deserialize)]
pub struct BloomRequest {
    block_number: u64,
    logs: String,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
pub struct Bloom {
    #[serde(with = "BigArray")]
    logs: [u8; 256],
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    LogsFilter,
}

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct LiteClient {
    logs_filter: UnorderedMap<BlockNumber, Bloom>,
}

#[near_bindgen]
impl LiteClient {
    #[init]
    #[private]
    pub fn init() -> Self {
        Self {
            logs_filter: UnorderedMap::new(StorageKey::LogsFilter),
        }
    }

    #[payable]
    pub fn validate(&mut self, block_number: BlockNumber, _proof: String) -> bool {
        let bloom = self.logs_filter.get(&block_number).unwrap().logs;
        let event_signature = near_sdk::env::keccak256("Locked(bytes32)".as_bytes());
        let mut default_bloom = utils::default();
        utils::accrue(&mut default_bloom, event_signature.try_into().unwrap());

        for i in 0..utils::BLOOM_SIZE {
            let a = bloom[i];
            let b = default_bloom[i];
            if (a & b) != b {
                return false;
            }
        }
        true
    }

    pub fn view_filter(&self, block_number: BlockNumber) -> Option<&Bloom> {
        self.logs_filter.get(&block_number)
    }

    pub fn insert_filter(&mut self, request: BloomRequest) {
        let decoded_blooom = base64::decode(&request.logs).unwrap();

        match self.logs_filter.get(&request.block_number) {
            Some(_) => panic!("Already added"),
            _ => self.logs_filter.insert(
                request.block_number,
                Bloom {
                    logs: decoded_blooom.try_into().unwrap(),
                },
            ),
        };
    }

    pub fn remove_filter(&mut self, block_number: BlockNumber) {
        self.logs_filter.remove(&block_number);
    }

    pub fn test(&self) -> &str {
        "Hello world!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;

    fn get_context() -> VMContextBuilder {
        let builder = VMContextBuilder::new();
        builder
    }
}
