use near_sdk::{
    self,
    borsh::{self, BorshDeserialize, BorshSerialize},
};
use near_sdk::{near_bindgen, BorshStorageKey};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    BlockHeader,
}

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct LiteClient {}

#[near_bindgen]
impl LiteClient {}

fn if_caller_is_admin() -> bool {
    return false;
}
