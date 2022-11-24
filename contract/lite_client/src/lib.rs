use near_sdk::{
    self,
    borsh::{self, BorshDeserialize, BorshSerialize},
    require,
    store::UnorderedMap,
    PanicOnDefault,
};
use near_sdk::{near_bindgen, BorshStorageKey};

pub type BlockNumber = u64;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    BlockHeaders,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct BlockHeader {
    logs_filter: Vec<u8>,
}

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct LiteClient {
    block_headhers: UnorderedMap<BlockNumber, BlockHeader>,
}

#[near_bindgen]
impl LiteClient {
    #[init]
    #[private]
    pub fn init() -> Self {
        Self {
            block_headhers: UnorderedMap::new(StorageKey::BlockHeaders),
        }
    }

    pub fn get_logs_filter(&self, block_number: BlockNumber) -> Vec<u8> {
        match self.block_headhers.get(&block_number) {
            Some(header) => header.logs_filter.clone(),
            _ => panic!("Logs filter doesn't exist"),
        }
    }

    pub fn add_block_header(&mut self, block_number: BlockNumber, logs_filter: Vec<u8>) {
        require!(if_caller_is_admin(), "Caller isn't owner");
        self.block_headhers
            .insert(block_number, BlockHeader { logs_filter });
    }

    pub fn remove_block_header(&mut self, block_number: BlockNumber) {
        require!(if_caller_is_admin(), "Caller isn't owner");
        self.block_headhers.remove(&block_number);
    }
}

fn if_caller_is_admin() -> bool {
    return true;
}
