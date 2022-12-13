use erc20::*;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, require, PanicOnDefault,
};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct FunCoin {
    token: ERC20,
}

#[near_bindgen]
impl FunCoin {
    #[init]
    #[private]
    pub fn init(name: String, symbol: String, decimals: u8, total_supply: u64) -> Self {
        Self {
            token: ERC20::init(name, symbol, decimals, total_supply),
        }
    }
}
