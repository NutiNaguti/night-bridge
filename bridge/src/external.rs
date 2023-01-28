use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    ext_contract,
    json_types::U128,
    AccountId,
};

pub const TGAS: u64 = 1_000_000_000_000;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ValidateRequest {
    pub block_number: u64,
    pub eth_bridge_address: String,
    pub event_signature: String,
    pub proof: String,
}

#[ext_contract(line_node)]
trait LiteNode {
    fn validate(&mut self, #[serializer(borsh)] request: ValidateRequest) -> bool;
}

#[ext_contract(fun_coin)]
trait FunCoin {
    fn mint(&mut self, to: AccountId, value: U128);
    fn burn(&mut self, account_id: AccountId, amount: U128);
}
