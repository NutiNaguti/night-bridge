use near_sdk::{ext_contract, json_types::U128, AccountId};

pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(line_node)]
trait LiteNode {
    fn validate(
        &mut self,
        block_number: u64,
        eth_contract_address: String,
        event_signature: String,
        proof: String,
    ) -> bool;
}

#[ext_contract(fun_coin)]
trait FunCoin {
    fn mint(&mut self, to: AccountId, value: U128);
    fn burn(&mut self, account_id: AccountId, amount: U128);
}
