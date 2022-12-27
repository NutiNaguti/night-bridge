use near_sdk::{ext_contract, AccountId};

pub const TGAS: u64 = 1_000_000_000_000;
// pub const NO_DEPOSIT: u128 = 0;
// pub const XCC_SUCCESS: u64 = 1;

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
    fn mint(&mut self, to: AccountId, value: u64);
    fn burn(&mut self, account_id: AccountId, amount: u64);
}
