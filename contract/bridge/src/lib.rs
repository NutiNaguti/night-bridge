use external::{fun_coin, line_node, TGAS};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::{self, predecessor_account_id};
use near_sdk::{self, collections::UnorderedSet, AccountId};
use near_sdk::{
    json_types, log, near_bindgen, require, BorshStorageKey, Gas, PanicOnDefault, Promise,
    PromiseError, ONE_NEAR,
};

mod external;

const VALIDATE_TRANSFER_FEE: u128 = ONE_NEAR;

#[derive(BorshSerialize, BorshDeserialize, BorshStorageKey)]
pub enum StorageKey {
    AdminList,
}

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Bridge {
    eth_event_signature: String,
    eth_bridge_address: String,
    eth_token_address: String,
    near_token_account: AccountId,
    lite_node_account: AccountId,
    admin_list: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl Bridge {
    #[init]
    #[private]
    pub fn init(
        eth_event_signature: String,
        eth_bridge_address: String,
        eth_token_address: String,
        near_token_account: AccountId,
        lite_node_account: AccountId,
    ) -> Self {
        Self {
            eth_event_signature,
            eth_bridge_address,
            eth_token_address,
            near_token_account,
            lite_node_account,
            admin_list: UnorderedSet::new(StorageKey::AdminList),
        }
    }

    #[payable]
    pub fn validate_transfer(
        &mut self,
        block_number: u64,
        receiver: AccountId,
        amount: u128,
        proof: String,
    ) -> Promise {
        let deposit = env::attached_deposit();
        require!(deposit >= VALIDATE_TRANSFER_FEE, "Not enougth funds");
        let promise = line_node::ext(self.lite_node_account.clone())
            .with_static_gas(Gas(5 * TGAS))
            .validate(
                block_number,
                self.eth_bridge_address.clone(),
                self.eth_event_signature.clone(),
                proof,
            );

        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(5 * TGAS))
                .with_attached_deposit(deposit / 2)
                .validate_callback(receiver, amount),
        )
    }

    #[private]
    pub fn validate_callback(
        &self,
        receiver: AccountId,
        amount: u128,
        #[callback_result] call_result: Result<bool, PromiseError>,
    ) -> Promise {
        if call_result.is_err() {
            panic!("There was an error contacting Lite Node");
        }

        let call_result = call_result.unwrap();
        log!("call_result: {}", call_result);

        if call_result {
            let deposit = env::attached_deposit();
            let promise = fun_coin::ext(self.near_token_account.clone())
                .with_static_gas(Gas(5 * TGAS))
                .with_attached_deposit(deposit)
                .ft_transfer(receiver, json_types::U128(amount), None);
            promise
        } else {
            panic!("Proof are invalid");
        }
    }

    pub fn check_admin(&self) -> bool {
        self.admin_list.contains(&predecessor_account_id())
    }

    pub fn set_token_address(&mut self, token_account: AccountId) {
        // self.token_account = token_account;
    }

    pub fn mint(&mut self, address: String, amount: u128) {
        unimplemented!()
    }

    pub fn burn(&mut self, amount: u128) {
        unimplemented!()
    }
}
